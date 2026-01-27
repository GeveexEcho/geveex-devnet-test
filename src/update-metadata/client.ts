import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { Metaplex, keypairIdentity } from "@metaplex-foundation/js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";

const wait = (ms: number) => new Promise(res => {
  const start = Date.now();
  while (Date.now() - start < ms) {}
  res(true);
});

async function runFixer() {
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  const wallet = (provider.wallet as any).payer;
  const metaplex = Metaplex.make(connection).use(keypairIdentity(wallet));

  const WHALE_URI = "https://raw.githubusercontent.com/GeveexEcho/geveex-devnet-test/refs/heads/main/assets/Whale/assets/metadata.json";
  const BELIEVERS_URI = "https://raw.githubusercontent.com/GeveexEcho/geveex-devnet-test/refs/heads/main/assets/believers/assets/metadata.json";
  const COMMUNITY_URI = "https://raw.githubusercontent.com/GeveexEcho/geveex-devnet-test/refs/heads/main/assets/community/assets/metadata.json";

  const ACTIVE_URI = WHALE_URI;

  const allTokens = await connection.getParsedTokenAccountsByOwner(
    wallet.publicKey,
    { programId: TOKEN_PROGRAM_ID }
  );

  console.log("Total tokens found: " + allTokens.value.length);

  for (const ta of allTokens.value) {
    const mint = new PublicKey(ta.account.data.parsed.info.mint);
    const amount = ta.account.data.parsed.info.tokenAmount.uiAmount;

    if (amount === 1) {
      try {
        const metadataPda = metaplex.nfts().pdas().metadata({ mint });
        const accountInfo = await connection.getAccountInfo(metadataPda);

        if (!accountInfo) {
          console.log("Updating: " + mint.toBase58());
          
          await metaplex.nfts().createSft({
            useExistingMint: mint,
            name: "Geveex Devnet NFT",
            symbol: "GEV",
            uri: ACTIVE_URI, 
            sellerFeeBasisPoints: 700,
          });
          
          await wait(800); 
        }
      } catch (e) {
        console.log("Error: " + e.message);
        await wait(1000);
      }
    }
  }
  console.log("Done.");
}

runFixer();

