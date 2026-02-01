// Client.ts for Solana Playground
import { 
  getAssociatedTokenAddressSync, 
  TOKEN_PROGRAM_ID, 
  ASSOCIATED_TOKEN_PROGRAM_ID 
} from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";

// Constants
const METADATA_URI = "https://raw.githubusercontent.com/GeveexEcho/geveex-devnet-test/refs/heads/main/programs/create-token/assets/metadata.json";
const TARGET_WALLET = new PublicKey("PWTPrTgMX2WM1gbsFSib7RrYiXPEvVR6t13n1zWht4G");
const TOKEN_METADATA_PROGRAM_ID = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

async function main() {
  console.log("🚀 Starting initialization on Solpg...");

  // 1. Generate a new Mint Keypair
  // In Solpg, web3 is globally available, but we can import it to be safe.
  const mintKeypair = new web3.Keypair();
  console.log("Mint Address:", mintKeypair.publicKey.toString());

  // 2. Fetch Metadata (Using fetch instead of axios for browser compatibility)
  console.log("Fetching metadata...");
  let metadataName = "Geveex Token"; 
  let metadataSymbol = "GVX"; 
  
  try {
    const response = await fetch(METADATA_URI);
    const json = await response.json();
    if(json.name) metadataName = json.name;
    if(json.symbol) metadataSymbol = json.symbol;
    console.log(`Metadata: ${metadataName} (${metadataSymbol})`);
  } catch (err) {
    console.log("⚠️ Could not fetch metadata, using defaults.");
  }

  // 3. Derive PDA for Metadata
  const [metadataAddress] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      mintKeypair.publicKey.toBuffer(),
    ],
    TOKEN_METADATA_PROGRAM_ID
  );

  // 4. Derive ATA for the Target Wallet
  const recipientAta = getAssociatedTokenAddressSync(
    mintKeypair.publicKey,
    TARGET_WALLET,
    false,
    TOKEN_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID
  );

  // 5. Send Transaction using `pg.program`
  try {
    const txHash = await pg.program.methods
      .initializeToken(
        metadataName,
        metadataSymbol,
        METADATA_URI
      )
      .accounts({
        payer: pg.wallet.publicKey,
        mintAccount: mintKeypair.publicKey,
        recipient: TARGET_WALLET,
        recipientTokenAccount: recipientAta,
        metadataAccount: metadataAddress,
        tokenProgram: TOKEN_PROGRAM_ID,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: web3.SystemProgram.programId,
        rent: web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([mintKeypair])
      .rpc();

    console.log("✅ Success!");
    console.log(`Minted 100,000 ${metadataSymbol} to ${TARGET_WALLET.toString()}`);
    console.log("Tx Hash:", txHash);
    
    // Log link for mobile users to click easily
    console.log(`https://explorer.solana.com/tx/${txHash}?cluster=devnet`);
  } catch (error) {
    console.error("❌ Failed:", error);
  }
}

// Run the function
main();
    
