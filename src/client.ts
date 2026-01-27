import * as anchor from "@coral-xyz/anchor";
import { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY, Keypair, Transaction } from "@solana/web3.js";
import { 
  TOKEN_PROGRAM_ID, 
  ASSOCIATED_TOKEN_PROGRAM_ID, 
  getAssociatedTokenAddress,
  createInitializeMintInstruction,
  MINT_SIZE,
  createAssociatedTokenAccountInstruction,
  createMintToInstruction
} from "@solana/spl-token";

// Config
const TARGET_WALLET = new PublicKey("PWTPrTgMX2WM1gbsFSib7RrYiXPEvVR6t13n1zWht4G");
const TM_PROG = new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

const COLLECTIONS = [
  { name: "Whale", symbol: "WHL", uri: "https://raw.githubusercontent.com/GeveexEcho/geveex-devnet-test/refs/heads/main/assets/Whale/assets/metadata.json", count: 1000 },
  { name: "Believers", symbol: "BLV", uri: "https://raw.githubusercontent.com/GeveexEcho/geveex-devnet-test/refs/heads/main/assets/believers/assets/metadata.json", count: 2000 },
  { name: "Community", symbol: "CMT", uri: "https://raw.githubusercontent.com/GeveexEcho/geveex-devnet-test/refs/heads/main/assets/community/assets/metadata.json", count: 3000 }
];

// Custom sleep for Playground
const wait = (ms: number) => new Promise(res => {
  const start = Date.now();
  while (Date.now() - start < ms) {}
  res(true);
});

async function run() {
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  const payer = (provider.wallet as any).payer as Keypair;

  console.log("Starting DIRECT minting to:", TARGET_WALLET.toBase58());

  for (const col of COLLECTIONS) {
    console.log(`\nCollection: ${col.name}`);
    for (let i = 0; i < col.count; i++) {
      try {
        const mint = Keypair.generate();
        const ata = await getAssociatedTokenAddress(mint.publicKey, TARGET_WALLET);
        const lamports = await connection.getMinimumBalanceForRentExemption(MINT_SIZE);

        const tx = new Transaction().add(
          // 1. Create Mint Account
          SystemProgram.createAccount({
            fromPubkey: payer.publicKey,
            newAccountPubkey: mint.publicKey,
            space: MINT_SIZE,
            lamports,
            programId: TOKEN_PROGRAM_ID,
          }),
          // 2. Initialize Mint
          createInitializeMintInstruction(mint.publicKey, 0, payer.publicKey, payer.publicKey),
          // 3. Create ATA for Target Wallet
          createAssociatedTokenAccountInstruction(payer.publicKey, ata, TARGET_WALLET, mint.publicKey),
          // 4. Mint 1 token to Target Wallet
          createMintToInstruction(mint.publicKey, ata, payer.publicKey, 1)
        );

        const signature = await connection.sendTransaction(tx, [payer, mint]);
        console.log(`[${i+1}/${col.count}] Minted: ${signature.substring(0, 8)}...`);

      } catch (e) {
        console.error(`Error:`, e.message);
        await wait(500);
      }
    }
  }
}

run();
          
