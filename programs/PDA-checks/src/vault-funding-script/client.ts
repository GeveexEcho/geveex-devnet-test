import { PublicKey } from "@solana/web3.js";
import { 
  getAssociatedTokenAddressSync, 
  createAssociatedTokenAccountInstruction,
  createTransferInstruction
} from "@solana/spl-token";

// Helper to nukes any invisible mobile characters
const clean = (addr: string) => addr.replace(/[^1-9A-HJ-NP-Za-km-z]/g, "");

(async () => {
  try {
    console.log("🧹 Cleaning addresses and starting...");

    const mintStr = clean("CRFVpZGWGDWbcajYrmB1GfTt4H9JtZNaFKkppLMfqRT7");
    const vestingStr = clean("OdwfT1bbEWVG9Ch77Y4fkTUQu7SFikSkv38T3kyTLXoH");
    const distributorStr = clean("HEZv2vTVBcoQwAqVXeo3Jc1ZuoVdEQ4P46xuU6yJanjx");

    const MINT = new PublicKey(mintStr);
    const VESTING_PDA = new PublicKey(vestingStr);
    const DISTRIBUTOR_PDA = new PublicKey(distributorStr);

    console.log("Verified Mint:", MINT.toBase58());

    const transaction = new web3.Transaction();
    const payer = pg.wallet.publicKey;

    // Derive ATAs for the PDAs
    const vestingAta = getAssociatedTokenAddressSync(MINT, VESTING_PDA, true);
    const distributorAta = getAssociatedTokenAddressSync(MINT, DISTRIBUTOR_PDA, true);
    const senderAta = getAssociatedTokenAddressSync(MINT, payer);

    // 1. Create the Token Accounts for the PDAs
    transaction.add(
      createAssociatedTokenAccountInstruction(payer, vestingAta, VESTING_PDA, MINT),
      createAssociatedTokenAccountInstruction(payer, distributorAta, DISTRIBUTOR_PDA, MINT)
    );

    // 2. Transfer logic
    const decimals = 8;
    const vestingAmt = BigInt(90000) * BigInt(10 ** decimals);
    const distributorAmt = BigInt(10000) * BigInt(10 ** decimals);

    transaction.add(
      createTransferInstruction(senderAta, vestingAta, payer, vestingAmt),
      createTransferInstruction(senderAta, distributorAta, payer, distributorAmt)
    );

    console.log("🚀 Sending transaction...");
    
    const tx = await web3.sendAndConfirmTransaction(
      pg.connection, 
      transaction, 
      [pg.wallet.keypair]
    );

    console.log("✅ Success! Vaults funded.");
    console.log("Tx Signature:", tx);

  } catch (err) {
    console.error("❌ Critical Error:", err.message);
    if (err.message.includes("base58")) {
        console.log("Check if you missed a character when copying. Lengths should be 43-44 chars.");
    }
  }
})();

