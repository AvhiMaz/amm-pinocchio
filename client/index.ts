import { sendAndConfirmTransaction, Transaction } from "@solana/web3.js";
import { PROGRAM_ID, RPC_URL } from "./helper/constants";
import { createAddLiquidityInstruction } from "./instructions/addLiquidity";
import { createInitializeInstruction } from "./instructions/initializer";
import { setupPoolAccounts } from "./helper/setup";
import { getConnection, loadKeypair } from "./helper/utils";
import { createSwapInstruction } from "./instructions/swap";
import { createWithdrawInstruction } from "./instructions/withdraw";
import { getAccount, getMint, createAccount } from "@solana/spl-token";

async function main() {
  const connection = getConnection(RPC_URL);
  const payer = loadKeypair("/Users/avhidotsol/.config/solana/id.json");

  console.log("Payer:", payer.publicKey.toString());
  console.log("Program ID:", PROGRAM_ID.toString());

  const setup = await setupPoolAccounts(connection, payer, PROGRAM_ID);

  console.log("Mint A:", setup.mintA.toString());
  console.log("Mint B:", setup.mintB.toString());
  console.log("Pool PDA:", setup.poolPda.toString());
  console.log("LP Mint:", setup.lpMint.toString());
  console.log("Vault A:", setup.vaultA.toString());
  console.log("Vault B:", setup.vaultB.toString());
  console.log("User Token A:", setup.userTokenA.toString());
  console.log("User Token B:", setup.userTokenB.toString());

  const initIx = createInitializeInstruction({
    programId: PROGRAM_ID,
    payer: payer.publicKey,
    ...setup,
    feeRate: 30,
    poolBump: setup.poolBump,
    lpMintBump: setup.lpMintBump,
  });

  const initTx = new Transaction().add(initIx);
  const initSig = await sendAndConfirmTransaction(connection, initTx, [payer]);
  console.log("Initialize signature:", initSig);

  const userLpToken = await createAccount(
    connection,
    payer,
    setup.lpMint,
    payer.publicKey,
  );
  console.log("User LP Token:", userLpToken.toString());

  const addLiqIx = createAddLiquidityInstruction({
    programId: PROGRAM_ID,
    payer: payer.publicKey,
    ...setup,
    userLpToken,
    amountA: BigInt(100_000),
    amountB: BigInt(100_000),
    minLpAmount: BigInt(0),
  });

  const addLiqTx = new Transaction().add(addLiqIx);
  const addLiqSig = await sendAndConfirmTransaction(connection, addLiqTx, [
    payer,
  ]);
  console.log("Add Liquidity signature:", addLiqSig);

  const userTokenAInfo = await getAccount(connection, setup.userTokenA);
  const userTokenBInfo = await getAccount(connection, setup.userTokenB);
  const userLpTokenInfo = await getAccount(connection, userLpToken);
  const vaultAInfo = await getAccount(connection, setup.vaultA);
  const vaultBInfo = await getAccount(connection, setup.vaultB);
  const lpMintInfo = await getMint(connection, setup.lpMint);

  console.log("\nAfter Add Liquidity:");
  console.log("User Token A balance:", userTokenAInfo.amount.toString());
  console.log("User Token B balance:", userTokenBInfo.amount.toString());
  console.log("User LP Token balance:", userLpTokenInfo.amount.toString());
  console.log("Vault A balance:", vaultAInfo.amount.toString());
  console.log("Vault B balance:", vaultBInfo.amount.toString());
  console.log("LP Mint supply:", lpMintInfo.supply.toString());

  const swapIx = createSwapInstruction({
    programId: PROGRAM_ID,
    payer: payer.publicKey,
    ...setup,
    amountIn: BigInt(10_000),
    minAmountOut: BigInt(9_000),
  });

  const swapTx = new Transaction().add(swapIx);
  const swapSig = await sendAndConfirmTransaction(connection, swapTx, [payer]);
  console.log("Swap signature:", swapSig);

  const userTokenAAfterSwap = await getAccount(connection, setup.userTokenA);
  const userTokenBAfterSwap = await getAccount(connection, setup.userTokenB);
  const vaultAAfterSwap = await getAccount(connection, setup.vaultA);
  const vaultBAfterSwap = await getAccount(connection, setup.vaultB);

  console.log("\nAfter Swap:");
  console.log("User Token A balance:", userTokenAAfterSwap.amount.toString());
  console.log("User Token B balance:", userTokenBAfterSwap.amount.toString());
  console.log("Vault A balance:", vaultAAfterSwap.amount.toString());
  console.log("Vault B balance:", vaultBAfterSwap.amount.toString());

  const withdrawIx = createWithdrawInstruction({
    programId: PROGRAM_ID,
    payer: payer.publicKey,
    ...setup,
    userLpToken,
    amountIn: BigInt(10_000),
    minAmountA: BigInt(9_000),
    minAmountB: BigInt(9_000),
  });

  const withdrawTx = new Transaction().add(withdrawIx);
  const withdrawSig = await sendAndConfirmTransaction(connection, withdrawTx, [
    payer,
  ]);
  console.log("Withdraw signature:", withdrawSig);

  const userTokenAFinal = await getAccount(connection, setup.userTokenA);
  const userTokenBFinal = await getAccount(connection, setup.userTokenB);
  const userLpTokenFinal = await getAccount(connection, userLpToken);
  const vaultAFinal = await getAccount(connection, setup.vaultA);
  const vaultBFinal = await getAccount(connection, setup.vaultB);
  const lpMintFinal = await getMint(connection, setup.lpMint);

  console.log("\nAfter Withdraw:");
  console.log("User Token A balance:", userTokenAFinal.amount.toString());
  console.log("User Token B balance:", userTokenBFinal.amount.toString());
  console.log("User LP Token balance:", userLpTokenFinal.amount.toString());
  console.log("Vault A balance:", vaultAFinal.amount.toString());
  console.log("Vault B balance:", vaultBFinal.amount.toString());
  console.log("LP Mint supply:", lpMintFinal.supply.toString());

  console.log("\nfuckkkkk yesssssss!");
}

main().catch(console.error);
