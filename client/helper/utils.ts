import { Connection, Keypair } from "@solana/web3.js";
import fs from "fs";

export function getConnection(url: string) {
  return new Connection(url, "confirmed");
}

export function loadKeypair(path: string): Keypair {
  const secretKey = JSON.parse(fs.readFileSync(path, "utf-8"));
  return Keypair.fromSecretKey(new Uint8Array(secretKey));
}
