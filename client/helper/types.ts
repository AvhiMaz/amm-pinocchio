import { PublicKey } from "@solana/web3.js";

export interface SetupResult {
  mintA: PublicKey;
  mintB: PublicKey;
  poolPda: PublicKey;
  lpMint: PublicKey;
  vaultA: PublicKey;
  vaultB: PublicKey;
  userTokenA: PublicKey;
  userTokenB: PublicKey;
  poolBump: number;
  lpMintBump: number;
}

export interface AddLiquidityParams {
  programId: PublicKey;
  payer: PublicKey;
  poolPda: PublicKey;
  lpMint: PublicKey;
  vaultA: PublicKey;
  vaultB: PublicKey;
  userTokenA: PublicKey;
  userTokenB: PublicKey;
  userLpToken: PublicKey;
  amountA: bigint;
  amountB: bigint;
  minLpAmount: bigint;
}

export interface InitializeParams {
  programId: PublicKey;
  payer: PublicKey;
  poolPda: PublicKey;
  mintA: PublicKey;
  mintB: PublicKey;
  lpMint: PublicKey;
  vaultA: PublicKey;
  vaultB: PublicKey;
  feeRate: number;
  poolBump: number;
  lpMintBump: number;
}

export interface SwapParams {
  programId: PublicKey;
  payer: PublicKey;
  poolPda: PublicKey;
  mintA: PublicKey;
  mintB: PublicKey;
  vaultA: PublicKey;
  vaultB: PublicKey;
  userTokenA: PublicKey;
  userTokenB: PublicKey;
  amountIn: bigint;
  minAmountOut: bigint;
}

export interface WithdrawParams {
  programId: PublicKey;
  payer: PublicKey;
  poolPda: PublicKey;
  lpMint: PublicKey;
  vaultA: PublicKey;
  vaultB: PublicKey;
  userLpToken: PublicKey;
  userTokenA: PublicKey;
  userTokenB: PublicKey;
  amountIn: bigint;
  minAmountA: bigint;
  minAmountB: bigint;
}
