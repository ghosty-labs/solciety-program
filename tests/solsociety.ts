import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Solsociety } from "../target/types/solsociety";
import { user } from ".";
import { assert } from "chai";

const program = anchor.workspace.Solsociety as Program<Solsociety>;

export const sendPost = async (user: any, tag: string, content: string) => {
  const newKeypair = anchor.web3.Keypair.generate();

  await program.methods
    .sendPost(tag, content)
    .accounts({
      post: newKeypair.publicKey,
      user: user.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers(user instanceof anchor.Wallet ? [newKeypair] : [user, newKeypair])
    .rpc();

  const post = await program.account.post.fetch(newKeypair.publicKey);
  return { publicKey: newKeypair.publicKey, account: post };
};

describe("Post", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.local());

  it("can send post", async () => {
    const post = await sendPost(user, "Test Post #1", "Test Tag #1");

    // assert.equal(post.account.(user as any).toBase58(), user.publicKey.toBase58());
  });
});
