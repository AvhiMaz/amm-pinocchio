import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { SystemProgram, TransactionInstruction } from "@solana/web3.js";
import { InitializeParams } from "../helper/types";

export function createInitializeInstruction(
  params: InitializeParams,
): TransactionInstruction {
  const initData = Buffer.alloc(5);
  initData.writeUInt8(0, 0);
  initData.writeUInt16LE(params.feeRate, 1);
  initData.writeUInt8(params.poolBump, 3);
  initData.writeUInt8(params.lpMintBump, 4);

  return new TransactionInstruction({
    programId: params.programId,
    keys: [
      { pubkey: params.payer, isSigner: true, isWritable: true },
      { pubkey: params.poolPda, isSigner: false, isWritable: true },
      { pubkey: params.mintA, isSigner: false, isWritable: false },
      { pubkey: params.mintB, isSigner: false, isWritable: false },
      { pubkey: params.lpMint, isSigner: false, isWritable: true },
      { pubkey: params.vaultA, isSigner: false, isWritable: true },
      { pubkey: params.vaultB, isSigner: false, isWritable: true },
      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },
      { pubkey: params.payer, isSigner: true, isWritable: true },
      { pubkey: params.poolPda, isSigner: false, isWritable: true },
    ],
    data: initData,
  });
}
