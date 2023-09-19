import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Solsociety } from "../target/types/solsociety";
import { createUser, user } from ".";
import { assert } from "chai";
import { Keypair, PublicKey } from "@solana/web3.js";
import { sendPost } from "./solsociety";

const program = anchor.workspace.Solsociety as Program<Solsociety>;

export const sendComment = async ({
  user,
  postParent,
  content,
  commentParent,
}: {
  user: any;
  postParent: PublicKey;
  content: String;
  commentParent: PublicKey;
}) => {
  const commentKeypair = Keypair.generate();

  await program.methods
    .sendComment(postParent, "Comment")
    .accounts({
      comment: commentKeypair.publicKey,
      user: user.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers(
      user instanceof anchor.Wallet ? [commentKeypair] : [user, commentKeypair]
    )
    .rpc();
};

describe("Comment", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.local());

  it("can send comment", async () => {
    const post: any = await sendPost(user, "tag", "Test Content #1");

    assert.equal(post.account.user.toBase58(), user.publicKey.toBase58());
    assert.equal(post.account.tag, "tag");
    assert.equal(post.account.content, "Test Content #1");
    assert.ok(post.account.timestamp);

    const comment: any = await sendComment({
      user,
      postParent: post.publicKey,
      content: "Comment",
      commentParent: null,
    });
    assert.equal(comment.account.post.toBase58(), post.publicKey.toBase58());
  });
});
