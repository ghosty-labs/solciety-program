const {
  setProvider,
  AnchorProvider,
  Program,
  getProvider,
  Wallet,
} = require("@coral-xyz/anchor");
const { Connection } = require("@solana/web3.js");
const fs = require("fs");
const { MongoClient, Logger, Decimal128, ObjectId } = require("mongodb");

const main = async () => {
  const mongoClient = new MongoClient(
    "mongodb+srv://wahdanaedo:yw3MMTWHSdxLxoAX@cluster0.srr7czq.mongodb.net/"
  );
  await mongoClient.connect();

  const solanaWeb3 = require("@solana/web3.js");

  const connection = new Connection(
    "https://aged-cool-shard.solana-devnet.discover.quiknode.pro/63e5d459890844fd35c95e5872eb460332d8f25d/"
  );
  for (let i = 0; i < 10; i++) {
    const keypairFile = fs.readFileSync(`../solana-wallet/${i}.json`);
    const solcietyKey = JSON.parse(keypairFile).slice(0, 32);
    // console.log(solcietyKey);
    // const solcietyKey = [
    //   169, 166, 40, 217, 177, 18, 14, 186, 194, 158, 156, 151, 115, 165, 232, 102,
    //   113, 63, 237, 136, 48, 178, 243, 57, 110, 14, 168, 199, 83, 10, 141, 89,
    //   145, 178, 219, 73, 132, 123, 133, 169, 38, 166, 29, 73, 223, 107, 58, 95,
    //   139, 67, 244, 226, 110, 52, 15, 242, 182, 53, 150, 162, 83, 35, 250, 104,
    // ].slice(0, 32);
    const userOne = solanaWeb3.Keypair.fromSeed(Uint8Array.from(solcietyKey));

    const wallet = new Wallet(userOne);
    const provider = new AnchorProvider(
      connection,
      wallet,
      AnchorProvider.defaultOptions()
    );

    // await connection.requestAirdrop(
    //   userOne.publicKey,
    //   1 * solanaWeb3.LAMPORTS_PER_SOL
    // );

    const user = provider.wallet;
    const idl = require("../idl/solciety.json");
    const programId = new solanaWeb3.PublicKey(
      "6sTexXR4daCeaGPL6dBpaVhadBMjU9fMkpUhSP4MGEEs"
    );

    const program = new Program(idl, programId, provider);

    const userTwo = solanaWeb3.Keypair.generate();

    // console.log(userOne.publicKey.toString());

    const socialMediaPosts = [
      "Enjoying a peaceful weekend getaway in the mountain",
      "Tried a new recipe today, and it was a success! ",
      "Just finished a great book any recommendations for my next read? ",
      "Spent the day at the beach, and I'm loving the sun and sand! ",
      "Netflix and chill kind of evening. Any movie suggestions? ",
      "Exploring a new city and making unforgettable memories.",
      "Sunday morning coffee is my favorite ritual. ",
      "Hiking through the forest, surrounded by the beauty of nature. ",
      "Feeling grateful for family and good times.",
      "A little progress each day adds up to big results. ",
    ];
    const postTags = [
      "nature",
      "foodie",
      "books",
      "beach",
      "movie",
      "travel",
      "coffee",
      "hike",
      "grate",
      "motiv",
    ];

    const post = await program.methods
      .sendPost(postTags[i], socialMediaPosts[i])
      .accounts({
        post: userTwo.publicKey,
        user: user.publicKey,
        systemProgram: solanaWeb3.SystemProgram.programId,
      })
      .signers([userTwo])
      .rpc();
    console.log(post);

    // await program.methods
    //   .sendComment(
    //     new solanaWeb3.PublicKey("Euc1iLfugbjSyQpZUkVwoGE13wx6tVz88pwYAEUhDZ6Y"),
    //     "Comment 1",
    //     new solanaWeb3.PublicKey("Euc1iLfugbjSyQpZUkVwoGE13wx6tVz88pwYAEUhDZ6Y")
    //   )
    //   .accounts({ comment: userTwo.publicKey })
    //   .signers([userTwo])
    //   .rpc();

    // await program.methods
    //   .followUser(
    //     new solanaWeb3.PublicKey("bZWUWpqTyY34aiwZ5czg2nPyxHUHtKpzCXwHa5qjs2j")
    //   )
    //   .accounts({
    //     follow: userTwo.publicKey,
    //     user: user.publicKey,
    //     systemProgram: solanaWeb3.SystemProgram.programId,
    //   })
    //   .signers([userTwo])
    //   .rpc();

    const tagBios = [
      "Exploring the beauty of the great outdoors 🌿",
      "Passionate about all things delicious 🍔",
      "A bookworm on a reading adventure 📚",
      "Forever chasing the sun and surf 🏖️",
      "A film fanatic with popcorn in hand 🎬",
      "Wandering the world one destination at a time ✈️",
      "Start each day with a cup of inspiration ☕",
      "Hiking through the wilderness, one step at a time 🌲",
      "Grateful for every moment and every smile 😊",
      "Small steps lead to big achievements 💪",
    ];
    mongoClient
      .db("solciety")
      .collection("profiles")
      .updateOne(
        { public_key: userOne.publicKey.toString() },
        {
          $set: {
            public_key: userOne.publicKey.toString(),
            __v: 0,
            alias: null,
            bio: tagBios[i],
            created_at: 1697197365709,
            has_new_post: false,
            has_notification: false,
            image: `https://api.dicebear.com/7.x/initials/png?seed=${userOne.publicKey.toString()}`,
            is_verified: true,
            total_follower: 0,
            total_following: 0,
            total_post: 1,
            updated_at: 1697200108526,
          },
        },
        { upsert: true }
      );

    // console.log("Processing : ", userOne.publicKey.toString());
  }

  process.exit(1);
};
main();
