const {
  setProvider,
  AnchorProvider,
  Program,
  getProvider,
  Wallet,
} = require("@coral-xyz/anchor");
const { Connection } = require("@solana/web3.js");

const main = async () => {
  const solanaWeb3 = require("@solana/web3.js");

  const connection = new Connection(
    "https://aged-cool-shard.solana-devnet.discover.quiknode.pro/63e5d459890844fd35c95e5872eb460332d8f25d/"
  );

  const solcietyKey = [
    169, 166, 40, 217, 177, 18, 14, 186, 194, 158, 156, 151, 115, 165, 232, 102,
    113, 63, 237, 136, 48, 178, 243, 57, 110, 14, 168, 199, 83, 10, 141, 89,
    145, 178, 219, 73, 132, 123, 133, 169, 38, 166, 29, 73, 223, 107, 58, 95,
    139, 67, 244, 226, 110, 52, 15, 242, 182, 53, 150, 162, 83, 35, 250, 104,
  ].slice(0, 32);
  const userOne = solanaWeb3.Keypair.fromSeed(Uint8Array.from(solcietyKey));

  const wallet = new Wallet(userOne);
  const provider = new AnchorProvider(
    connection,
    wallet,
    AnchorProvider.defaultOptions()
  );

  const user = provider.wallet;

  const idl = require("../idl/solciety.json");
  const programId = new solanaWeb3.PublicKey(
    "6sTexXR4daCeaGPL6dBpaVhadBMjU9fMkpUhSP4MGEEs"
  );

  const program = new Program(idl, programId, provider);

  const userTwo = solanaWeb3.Keypair.generate();

  // const post = await program.methods
  //   .sendPost("Tag33", "Content Example 33")
  //   .accounts({
  //     post: userTwo.publicKey,
  //     user: user.publicKey,
  //     systemProgram: solanaWeb3.SystemProgram.programId,
  //   })
  //   .signers([userTwo])
  //   .rpc();

  await program.methods
    .sendComment(
      new solanaWeb3.PublicKey("AVcoemRHVnuytzQHmDcPd3p8ngfA751z2F9mxZoddrWb"),
      "Comment 5 😡😄",
      new solanaWeb3.PublicKey("AVcoemRHVnuytzQHmDcPd3p8ngfA751z2F9mxZoddrWb")
    )
    .accounts({ comment: userTwo.publicKey })
    .signers([userTwo])
    .rpc();

  // const sign = await connection.getSignatureStatus('2BsbV6FfojtvQsWRkrBueXcoPg8J7yV5fTTs9u1xrjTXWFuFkH2rjkeNu6DUDAmkRutcEeBGRjqjYf6YP6PjYjjh')
  // console.log("POST : ", post);
};
main();
