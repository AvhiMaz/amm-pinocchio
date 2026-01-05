import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  ACCOUNT_SIZE,
  createMint,
  createAccount,
  mintTo,
  getAccount,
  getMint,
  createInitializeAccountInstruction,
} from "@solana/spl-token";
import fs from "fs";

const PROGRAM_ID = new PublicKey(
  "ABNkPBUxPGKj2JWJSb8Nnmqw1RTjDmPYBHA8oQvkNGTJ",
);
const POOL_SEED = "pool";
const LP_MINT_SEED = "lp_mint";

async function main() {
  const connection = new Connection(
    "https://api.devnet.solana.com",
    "confirmed",
  );

  const secretKey = JSON.parse(
    fs.readFileSync("/Users/avhidotsol/.config/solana/id.json", "utf-8"),
  );
  const payer = Keypair.fromSecretKey(new Uint8Array(secretKey));

  console.log("Payer:", payer.publicKey.toString());
  console.log("Program ID:", PROGRAM_ID.toString());

  const mintA = await createMint(connection, payer, payer.publicKey, null, 6);
  console.log("Mint A:", mintA.toString());

  const mintB = await createMint(connection, payer, payer.publicKey, null, 6);
  console.log("Mint B:", mintB.toString());

  const [poolPda, poolBump] = PublicKey.findProgramAddressSync(
    [Buffer.from(POOL_SEED), mintA.toBuffer(), mintB.toBuffer()],
    PROGRAM_ID,
  );
  console.log("\nPool PDA:", poolPda.toString(), "Bump:", poolBump);

  const [lpMint, lpMintBump] = PublicKey.findProgramAddressSync(
    [Buffer.from(LP_MINT_SEED), poolPda.toBuffer()],
    PROGRAM_ID,
  );
  console.log("LP Mint PDA:", lpMint.toString(), "Bump:", lpMintBump);

  const vaultAKeypair = Keypair.generate();
  const vaultBKeypair = Keypair.generate();

  const vaultA = vaultAKeypair.publicKey;
  const vaultB = vaultBKeypair.publicKey;

  const lamports =
    await connection.getMinimumBalanceForRentExemption(ACCOUNT_SIZE);

  const createVaultATx = new Transaction().add(
    SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: vaultA,
      space: ACCOUNT_SIZE,
      lamports,
      programId: TOKEN_PROGRAM_ID,
    }),
    createInitializeAccountInstruction(
      vaultA,
      mintA,
      poolPda,
      TOKEN_PROGRAM_ID,
    ),
  );
  await sendAndConfirmTransaction(connection, createVaultATx, [
    payer,
    vaultAKeypair,
  ]);
  console.log("Vault A created and initialized:", vaultA.toString());

  const createVaultBTx = new Transaction().add(
    SystemProgram.createAccount({
      fromPubkey: payer.publicKey,
      newAccountPubkey: vaultB,
      space: ACCOUNT_SIZE,
      lamports,
      programId: TOKEN_PROGRAM_ID,
    }),
    createInitializeAccountInstruction(
      vaultB,
      mintB,
      poolPda,
      TOKEN_PROGRAM_ID,
    ),
  );
  await sendAndConfirmTransaction(connection, createVaultBTx, [
    payer,
    vaultBKeypair,
  ]);
  console.log("Vault B created and initialized:", vaultB.toString());

  const userTokenA = await createAccount(
    connection,
    payer,
    mintA,
    payer.publicKey,
  );
  await mintTo(connection, payer, mintA, userTokenA, payer, 1_000_000);
  console.log("User Token A:", userTokenA.toString(), "(1,000,000 tokens)");

  const userTokenB = await createAccount(
    connection,
    payer,
    mintB,
    payer.publicKey,
  );
  await mintTo(connection, payer, mintB, userTokenB, payer, 1_000_000);
  console.log("User Token B:", userTokenB.toString(), "(1,000,000 tokens)");

  const feeRate = 30;
  const initData = Buffer.alloc(5);
  initData.writeUInt8(0, 0);
  initData.writeUInt16LE(feeRate, 1);
  initData.writeUInt8(poolBump, 3);
  initData.writeUInt8(lpMintBump, 4);

  const initIx = new TransactionInstruction({
    programId: PROGRAM_ID,
    keys: [
      { pubkey: payer.publicKey, isSigner: true, isWritable: true },
      { pubkey: poolPda, isSigner: false, isWritable: true },
      { pubkey: mintA, isSigner: false, isWritable: false },
      { pubkey: mintB, isSigner: false, isWritable: false },
      { pubkey: lpMint, isSigner: false, isWritable: true },
      { pubkey: vaultA, isSigner: false, isWritable: true },
      { pubkey: vaultB, isSigner: false, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
    ],
    data: initData,
  });

  const initTx = new Transaction().add(initIx);
  const initSig = await sendAndConfirmTransaction(connection, initTx, [payer]);
  console.log("Initialize signature:", initSig);

  const userLpToken = await createAccount(
    connection,
    payer,
    lpMint,
    payer.publicKey,
  );
  console.log("User LP Token:", userLpToken.toString());

  const amountA = BigInt(100_000);
  const amountB = BigInt(100_000);
  const minLpAmount = BigInt(0);

  const accountsDebug = [
    { name: "payer", pubkey: payer.publicKey },
    { name: "poolPda", pubkey: poolPda },
    { name: "lpMint", pubkey: lpMint },
    { name: "vaultA", pubkey: vaultA },
    { name: "vaultB", pubkey: vaultB },
    { name: "userTokenA", pubkey: userTokenA },
    { name: "userTokenB", pubkey: userTokenB },
    { name: "userLpToken", pubkey: userLpToken },
    { name: "TOKEN_PROGRAM_ID", pubkey: TOKEN_PROGRAM_ID },
  ];

  console.log("Accounts:");
  for (const acc of accountsDebug) {
    console.log(`  ${acc.name}: ${acc.pubkey.toString()}`);
  }

  for (let i = 0; i < accountsDebug.length; i++) {
    for (let j = i + 1; j < accountsDebug.length; j++) {
      if (accountsDebug[i].pubkey.equals(accountsDebug[j].pubkey)) {
        console.error(
          `DUPLICATE: ${accountsDebug[i].name} and ${accountsDebug[j].name} have the same pubkey!`,
        );
      }
    }
  }

  const addLiqData = Buffer.alloc(25);
  addLiqData.writeUInt8(1, 0); // discriminator
  addLiqData.writeBigUInt64LE(amountA, 1);
  addLiqData.writeBigUInt64LE(amountB, 9);
  addLiqData.writeBigUInt64LE(minLpAmount, 17);

  const addLiqIx = new TransactionInstruction({
    programId: PROGRAM_ID,
    keys: [
      { pubkey: payer.publicKey, isSigner: true, isWritable: true },
      { pubkey: poolPda, isSigner: false, isWritable: true },
      { pubkey: lpMint, isSigner: false, isWritable: true },
      { pubkey: vaultA, isSigner: false, isWritable: true },
      { pubkey: vaultB, isSigner: false, isWritable: true },
      { pubkey: userTokenA, isSigner: false, isWritable: true },
      { pubkey: userTokenB, isSigner: false, isWritable: true },
      { pubkey: userLpToken, isSigner: false, isWritable: true },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
    ],
    data: addLiqData,
  });

  const addLiqTx = new Transaction().add(addLiqIx);
  const addLiqSig = await sendAndConfirmTransaction(connection, addLiqTx, [
    payer,
  ]);
  console.log("Add Liquidity signature:", addLiqSig);

  const userTokenAInfo = await getAccount(connection, userTokenA);
  const userTokenBInfo = await getAccount(connection, userTokenB);
  const userLpTokenInfo = await getAccount(connection, userLpToken);
  const vaultAInfo = await getAccount(connection, vaultA);
  const vaultBInfo = await getAccount(connection, vaultB);
  const lpMintInfo = await getMint(connection, lpMint);

  console.log(
    "User Token A balance:",
    userTokenAInfo.amount.toString(),
    "(should be 900,000)",
  );
  console.log(
    "User Token B balance:",
    userTokenBInfo.amount.toString(),
    "(should be 900,000)",
  );
  console.log("User LP Token balance:", userLpTokenInfo.amount.toString());
  console.log(
    "Vault A balance:",
    vaultAInfo.amount.toString(),
    "(should be 100,000)",
  );
  console.log(
    "Vault B balance:",
    vaultBInfo.amount.toString(),
    "(should be 100,000)",
  );
  console.log("LP Mint supply:", lpMintInfo.supply.toString());

  const amountIn = BigInt(10_000);
  const minAmountOut = BigInt(9_000);

  const swapData = Buffer.alloc(17);
  swapData.writeUInt8(2, 0);
  swapData.writeBigUInt64LE(amountIn, 1);
  swapData.writeBigUInt64LE(minAmountOut, 9);

  const swapIx = new TransactionInstruction({
    programId: PROGRAM_ID,
    keys: [
      { pubkey: payer.publicKey, isSigner: true, isWritable: true },
      { pubkey: poolPda, isSigner: false, isWritable: true },
      { pubkey: mintA, isSigner: false, isWritable: false },
      { pubkey: mintB, isSigner: false, isWritable: false },
      { pubkey: vaultA, isSigner: false, isWritable: true },
      { pubkey: vaultB, isSigner: false, isWritable: true },
      { pubkey: userTokenA, isSigner: false, isWritable: true },
      { pubkey: userTokenB, isSigner: false, isWritable: true },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
    ],
    data: swapData,
  });

  const swapTx = new Transaction().add(swapIx);
  const swapSig = await sendAndConfirmTransaction(connection, swapTx, [payer]);
  console.log("\nSwap signature:", swapSig);

  const userTokenAAfter = await getAccount(connection, userTokenA);
  const userTokenBAfter = await getAccount(connection, userTokenB);
  const vaultAAfter = await getAccount(connection, vaultA);
  const vaultBAfter = await getAccount(connection, vaultB);

  console.log("\nSwap Results:");
  console.log("User Token A balance:", userTokenAAfter.amount.toString());
  console.log("User Token B balance:", userTokenBAfter.amount.toString());
  console.log("Vault A balance:", vaultAAfter.amount.toString());
  console.log("Vault B balance:", vaultBAfter.amount.toString());

  console.log("\nFUCCKKKKKKKKK YESSSSSSSSSS!!!!!!!");
}

main().catch(console.error);
