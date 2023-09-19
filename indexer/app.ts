import { Connection, PublicKey, LAMPORTS_PER_SOL } from "@solana/web3.js";

const WSS_ENDPOINT =
  "wss://aged-cool-shard.solana-devnet.discover.quiknode.pro/63e5d459890844fd35c95e5872eb460332d8f25d/";
const HTTP_ENDPOINT =
  "https://aged-cool-shard.solana-devnet.discover.quiknode.pro/63e5d459890844fd35c95e5872eb460332d8f25d/";
const solanaConnection = new Connection(HTTP_ENDPOINT, {
  wsEndpoint: WSS_ENDPOINT,
});
const sleep = (ms: number) => {
  return new Promise((resolve) => setTimeout(resolve, ms));
};

(async () => {
  const ACCOUNT_TO_WATCH = new PublicKey(
    "6sTexXR4daCeaGPL6dBpaVhadBMjU9fMkpUhSP4MGEEs"
  );
  const subscriptionId = solanaConnection.onLogs(ACCOUNT_TO_WATCH, (result) => {
    console.log(result);
  });
  console.log("Starting web socket, subscription ID: ", subscriptionId);
  await sleep(10000); //Wait 10 seconds for Socket Testing
})();
