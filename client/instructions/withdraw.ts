import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { TransactionInstruction } from "@solana/web3.js";
import { WithdrawParams } from "../helper/types";

export function createWithdrawInstruction(
  params: WithdrawParams,
): TransactionInstruction {
  const data = Buffer.alloc(25);
  data.writeUInt8(3, 0);
  data.writeBigUInt64LE(params.amountIn, 1);
  data.writeBigUInt64LE(params.minAmountA, 9);
  data.writeBigUInt64LE(params.minAmountB, 17);

  return new TransactionInstruction({
    programId: params.programId,
    keys: [
      { pubkey: params.payer, isSigner: true, isWritable: true },
      { pubkey: params.poolPda, isSigner: false, isWritable: true },
      { pubkey: params.lpMint, isSigner: false, isWritable: true },
      { pubkey: params.vaultA, isSigner: false, isWritable: true },
      { pubkey: params.vaultB, isSigner: false, isWritable: true },
      { pubkey: params.userLpToken, isSigner: false, isWritable: true },
      { pubkey: params.userTokenA, isSigner: false, isWritable: true },
      { pubkey: params.userTokenB, isSigner: false, isWritable: true },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
    ],
    data,
  });
}
