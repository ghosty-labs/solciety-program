import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Solsociety } from "../target/types/solsociety";
import { createUser, user } from ".";
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
    const post: any = await sendPost(user, "tag", "Test Content #1");

    assert.equal(post.account.user.toBase58(), user.publicKey.toBase58());
    assert.equal(post.account.tag, "tag");
    assert.equal(post.account.content, "Test Content #1");
    assert.ok(post.account.timestamp);
  });

  it("can send and update post", async () => {
    const newUser = await createUser();

    const post = await sendPost(
      newUser,
      "before",
      "Test Content Send and Update #1"
    );

    assert.equal(post.account.user.toBase58(), newUser.publicKey.toBase58());
    assert.equal(post.account.tag, "before");
    assert.equal(post.account.content, "Test Content Send and Update #1");
    assert.ok(post.account.timestamp);

    await program.methods
      .updatePost("after", "Updated Test Content Send and Update #1")
      .accounts({ post: post.publicKey, user: newUser.publicKey })
      .signers([newUser])
      .rpc();

    const updatedPost: any = await program.account.post.fetch(post.publicKey);
    assert.equal(updatedPost.tag, "after");
    assert.equal(
      updatedPost.content,
      "Updated Test Content Send and Update #1"
    );
    assert.deepEqual(updatedPost.state, { edited: {} });
  });

  it("can send and delete post", async () => {
    const userOne = await createUser();

    const post = await sendPost(
      userOne,
      "deletetag",
      "Test Content Send and Delete #1"
    );

    assert.equal(post.account.user.toBase58(), userOne.publicKey.toBase58());
    assert.equal(post.account.tag, "deletetag");
    assert.equal(post.account.content, "Test Content Send and Delete #1");
    assert.ok(post.account.timestamp);

    await program.methods
      .deletePost()
      .accounts({ post: post.publicKey, user: userOne.publicKey })
      .signers([userOne])
      .rpc();

    const deletedPost = await program.account.post.fetch(post.publicKey);
    assert.equal(deletedPost.tag, "[deleted]");
    assert.equal(deletedPost.content, "");
  });
});
