import {expect,test} from 'bun:test';
import { Keypair,Connection, SystemProgram,Transaction} from '@solana/web3.js';
import { COUNTER_SIZE } from './types';


let adminAccount = Keypair.generate();
let dataAccount = Keypair.generate();


test("Account Generation", async ()=>{
    const connection = new Connection("https://solana-devnet.g.alchemy.com/v2/-wgX0L1sP7MA475YuImcVvf6fB4ymZQx", "confirmed");

    const txn = await connection.requestAirdrop(adminAccount.publicKey, 1000000000);
    await connection.confirmTransaction(txn);

    const data = await connection.getAccountInfo(adminAccount.publicKey);

    console.log(data);
    //airdrop done


    const lamport = await connection.getMinimumBalanceForRentExemption(COUNTER_SIZE);

    const ix = SystemProgram.createAccount({
        fromPubkey: adminAccount.publicKey,
        newAccountPubkey: dataAccount.publicKey,
        lamports: lamport,
        space: COUNTER_SIZE,
        programId: SystemProgram.programId
    })

    const  createAccountTxn = new Transaction();
    createAccountTxn.add(ix);

    const signature = await connection.sendTransaction(createAccountTxn, [adminAccount, dataAccount]);

    await connection.confirmTransaction(signature);

    console.log(`Data account created with public key: ${dataAccount.publicKey.toBase58()}`);


})



