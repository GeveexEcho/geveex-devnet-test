import { PublicKey } from "@solana/web3.js";
import { getAssociatedTokenAddressSync, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";

// Constants from your request
const MINT = new PublicKey("CRFVpZGWGDWbcajYrmB1GfTt4H9JtZNaFKkppLMfqRT7");
const TEAM_WALLET = new PublicKey("PWTPrTgMX2WM1gbsFSib7RrYiXPEvVR6t13n1zWht4G");

const USER_WALLETS = [
  new PublicKey("4JiaFex7YiE76fi1USe7uhaaUea1cnKN9VdX4LCon62h"),
  new PublicKey("FRZoKcnEhSLnGh2pGq3GztrTL2Qs92j8ksgEAWPKwYnu"),
  new PublicKey("FUUcei1RvR1xNJ3T1togn8fsb7NxoCrrQ7Wn2qn7q7ja"),
  new PublicKey("EAMo135E3HhZfTSdr5cRGx4EVwGJ7bm7GFtXNEistaQn")
];

// Wrapping in an async function to fix the 'await' error
(async () => {
  try {
    console.log("🚀 Initializing Vesting Client...");

    // 1. Derive PDAs
    const [vestingVault] = PublicKey.findProgramAddressSync(
      [Buffer.from("vesting")],
      pg.program.programId
    );
    const [distributorVault] = PublicKey.findProgramAddressSync(
      [Buffer.from("distributor")],
      pg.program.programId
    );

    console.log("Vesting Vault PDA:", vestingVault.toBase58());
    console.log("Distributor Vault PDA:", distributorVault.toBase58());

    // 2. Initialize State Account
    const stateKeypair = web3.Keypair.generate();
    const initTx = await pg.program.methods
      .initialize()
      .accounts({
        state: stateKeypair.publicKey,
        team: pg.wallet.publicKey,
        mint: MINT,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([stateKeypair])
      .rpc();

    console.log("✅ State Initialized. Tx:", initTx);

    // 3. Setup Token Accounts for the Distribution call
    // Note: We derive the ATAs for the users
    const userAtas = USER_WALLETS.map(wallet => 
      getAssociatedTokenAddressSync(MINT, wallet)
    );
    
    const teamAta = getAssociatedTokenAddressSync(MINT, TEAM_WALLET);

    console.log("--- NEXT STEPS ---");
    console.log("1. Send 90,000 tokens to Vesting Vault PDA");
    console.log("2. Send 10,000 tokens to Distributor Vault PDA");
    console.log("3. Once funded, call 'distributeInitial' function.");

  } catch (err) {
    console.error("❌ Error:", err);
  }
})();
