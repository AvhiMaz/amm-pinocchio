import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { TransactionInstruction } from "@solana/web3.js";
import { SwapParams } from "../helper/types";

export function createSwapInstruction(
  params: SwapParams,
): TransactionInstruction {
  const data = Buffer.alloc(17);
  data.writeUInt8(2, 0);
  data.writeBigUInt64LE(params.amountIn, 1);
  data.writeBigUInt64LE(params.minAmountOut, 9);

  return new TransactionInstruction({
    programId: params.programId,
    keys: [
      { pubkey: params.payer, isSigner: true, isWritable: true },
      { pubkey: params.poolPda, isSigner: false, isWritable: true },
      { pubkey: params.mintA, isSigner: false, isWritable: false },
      { pubkey: params.mintB, isSigner: false, isWritable: false },
      { pubkey: params.vaultA, isSigner: false, isWritable: true },
      { pubkey: params.vaultB, isSigner: false, isWritable: true },
      { pubkey: params.userTokenA, isSigner: false, isWritable: true },
      { pubkey: params.userTokenB, isSigner: false, isWritable: true },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
    ],
    data,
  });
}
