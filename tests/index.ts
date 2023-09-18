import * as anchor from "@coral-xyz/anchor";
import { Keypair } from "@solana/web3.js";
import { Solsociety } from "../target/types/solsociety";

export const provider = anchor.AnchorProvider.local();
export const user = provider.wallet;

anchor.setProvider(anchor.AnchorProvider.local());

export const createUser = async () => {
  try {
    const userKeypair = Keypair.generate();
    const userSignature = await provider.connection.requestAirdrop(
      userKeypair.publicKey,
      10 * anchor.web3.LAMPORTS_PER_SOL
    );

    const latestBlockHash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      blockhash: latestBlockHash.blockhash,
      lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
      signature: userSignature,
    });

    return userKeypair;
  } catch (error) {
    throw error;
  }
};

export const createUsers = (num: number) => {
  let promises = [];
  for (let i = 0; i < num; i++) promises.push(createUser());
  return Promise.all(promises);
};
