import {
  Connection,
  Keypair,
  PublicKey,
  sendAndConfirmTransaction,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
import {
  createMint,
  createAccount,
  mintTo,
  ACCOUNT_SIZE,
  createInitializeAccountInstruction,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { LP_MINT_SEED, POOL_SEED, PROGRAM_ID } from "./constants";
import { SetupResult } from "./types";

export async function setupPoolAccounts(
  connection: Connection,
  payer: Keypair,
  _programId: PublicKey,
): Promise<SetupResult> {
  const mintA = await createMint(connection, payer, payer.publicKey, null, 6);
  const mintB = await createMint(connection, payer, payer.publicKey, null, 6);

  const [poolPda, poolBump] = PublicKey.findProgramAddressSync(
    [Buffer.from(POOL_SEED), mintA.toBuffer(), mintB.toBuffer()],
    PROGRAM_ID,
  );

  const [lpMint, lpMintBump] = PublicKey.findProgramAddressSync(
    [Buffer.from(LP_MINT_SEED), poolPda.toBuffer()],
    PROGRAM_ID,
  );

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

  const userTokenA = await createAccount(
    connection,
    payer,
    mintA,
    payer.publicKey,
  );
  await mintTo(connection, payer, mintA, userTokenA, payer, 1_000_000);

  const userTokenB = await createAccount(
    connection,
    payer,
    mintB,
    payer.publicKey,
  );
  await mintTo(connection, payer, mintB, userTokenB, payer, 1_000_000);

  return {
    mintA,
    mintB,
    poolPda,
    lpMint,
    vaultA,
    vaultB,
    userTokenA,
    userTokenB,
    poolBump,
    lpMintBump,
  };
}
