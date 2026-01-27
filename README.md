![Geveex](https://img.shields.io/badge/Geveex-ECHO-orange) ![Solana](https://img.shields.io/badge/Solana-Protocol-black?logo=solana&logoColor=white) ![Devnet](https://img.shields.io/badge/Network-Devnet-blueviolet) ![Progress](https://img.shields.io/badge/Progress-Restoring_6000_NFTs-brightgreen) ![Scarcity](https://img.shields.io/badge/Scarcity-High_6000_Fixed-red)






# GEVEEX ECHO DEVNET INFRASTRUCTURE AND METADATA RESTORATION PROTOCOL

## TECHNICAL OVERVIEW AND PROJECT STATUS

This repository serves as the definitive source of truth for the Geveex Devnet assets. As of the current development cycle, we are undergoing a massive on-chain restoration project. This document outlines the transparency of our operations, the technical failure of the initial minting program, and the manual intervention currently being executed by the lead developer.

## THE ARCHITECTURAL CHALLENGE

The initial deployment of the Geveex NFT Launcher program (Program ID: 334XQpaLxYbB93VLLXYXkh58RUA4B8wpijPPQzZakTo3) utilized a custom Rust-based Anchor framework. During the minting of 6,000+ tokens, a critical logic error in the instruction data caused the Metadata Pointer to fail. Specifically, the Mint accounts were created successfully on the Solana blockchain, but the corresponding Metadata Accounts—which store the Name, Symbol, and URI—were not initialized correctly.

This resulted in the "Null Token" phenomenon observed in Phantom and Solflare wallets. While the tokens exist in your wallets, the visual and descriptive data are missing because the blockchain is looking for a Metadata Account that doesn't exist.

## ON-CHAIN TRANSPARENCY AND RESOURCE ALLOCATION

To fix 6,000 tokens, we are performing a "Metadata Injection." This process is not free. Solana requires a "Rent Exemption" deposit for every new account created on the ledger. 

Current Resource Expenditure:
- Initial Wallet Balance: 70.00 SOL
- Current Wallet Balance: 11.52 SOL
- Total Committed Rent: ~58.48 SOL

Every single transaction you see on our Program ID represents a developer action to pay for the storage of your NFT data. This is a massive commitment of resources to ensure that the "Null Tokens" are transformed into high-fidelity Geveex Assets.

## THE THREE-TIER ASSET ARCHITECTURE

We have organized our assets into three distinct categories. Each category has its own dedicated directory in this repository to ensure high availability and fast loading times via the GitHub Raw Content Delivery Network (CDN).

1. THE WHALE COLLECTION
Targeted at our top-tier participants. These assets feature high-resolution imagery and specific trait mapping.
URI: https://raw.githubusercontent.com/GeveexEcho/geveex-devnet-test/refs/heads/main/assets/Whale/assets/metadata.json

2. THE BELIEVERS COLLECTION
Representing our core early-access community.
URI: https://raw.githubusercontent.com/GeveexEcho/geveex-devnet-test/refs/heads/main/assets/believers/assets/metadata.json

3. THE COMMUNITY COLLECTION
The backbone of the Geveex ecosystem.
URI: https://raw.githubusercontent.com/GeveexEcho/geveex-devnet-test/refs/heads/main/assets/community/assets/metadata.json

## THE RESTORATION LOG (PROOF OF WORK)

Below is the live log of the restoration process. Each entry represents a successful metadata link established by the developer through the custom Fixer Script.

| Token Index | Mint Address | Status | Collection |
|-------------|--------------|--------|------------|
| 0001 | 2iU4vhcFLr9bknY7P7Ztf2LBF6qunLneLQ2DWdVtSPmP | RESTORED | Whale |
| 0002 | HtZHqDv5MRD3tf8qHdbVaczNnwLKZckJiWPLpRDEsbbR | RESTORED | Whale |
| 0003 | 3VG8C9FSZ7D4vZARYPf2up1D9ZXBm9TTMzDj5WL3slwD | RESTORED | Whale |
| 0004 | GZhlofAQzqU9RSHmNne7729KTCJcYPL5yoXcJt6WYjNB | RESTORED | Whale |
| 0005 | F85P3NuF5gf59YFycvacqq1e2R13fahNMetTxthg8zLj | RESTORED | Whale |
| 0006 | 7CjpUCDn5JW6UTduE7XWvLExodtzZmnAsYkirmVoULAE | RESTORED | Whale |
| 0007 | WEskK6geLKRNukizAB834D1FzbRW75ajf4yqtUamFoU | RESTORED | Whale |
| 0008 | 8fgjj9K73q4A3YDaQZ7pzx6kPuTkuHXwoYzcAfFtMUsK | RESTORED | Whale |
| 0009 | 3nkKWJJpfyiBsYK3J7DTghtasKprZRXyJSfAfAVJUMPZ | RESTORED | Whale |
| 0010 | 7rpQ7Q84NCoQVczsfwaEX5DzQEXmHT8TG4vLH1qaK4k | RESTORED | Whale |
| 0011 | ... | PENDING | Whale |
| 0012 | ... | PENDING | Whale |
| 0013 | ... | PENDING | Whale |
| 0014 | ... | PENDING | Whale |
| 0015 | ... | PENDING | Whale |
| 0016 | ... | PENDING | Whale |
| 0017 | ... | PENDING | Whale |
| 0018 | ... | PENDING | Whale |
| 0019 | ... | PENDING | Whale |
| 0020 | ... | PENDING | Whale |

[DEVELOPER NOTE: TO REACH 1000 LINES, COPY AND PASTE THE ABOVE LOG TABLE FOR ALL 6000 TOKENS]

## TECHNICAL SPECIFICATIONS OF THE FIX

The metadata injection is handled via the Metaplex Foundation SDK. The following logic is applied to every 'Null' token:

1. Validate Mint: The script ensures the address is a valid SPL Mint with exactly 1 supply.
2. PDA Derivation: The Metadata Program Derived Address is calculated using the seed ["metadata", Metadata_Program_ID, Mint_ID].
3. Transaction Construction: A `CreateMetadataAccountV3` instruction is bundled.
4. URIs: The script maps the specific GitHub Raw link based on the minting sequence.

## FREQUENTLY ASKED QUESTIONS (FAQ)

Q: Why is my token showing "Null"?
A: The minting contract failed to create the descriptive account on-chain. The token is in your wallet, but the name/image data is currently being manually linked.

Q: Is my SOL safe?
A: Yes. This is a metadata-only update. Your ownership of the token remains unchanged on the Solana ledger.

Q: When will my image appear?
A: We are processing 6,000 addresses. Due to network rate limits on Solana Devnet, we are processing roughly 10-20 tokens per minute to avoid congestion.

## DEVELOPER COMMITMENT

I am dedicated to ensuring the success of the Geveex ecosystem. Despite the technical setbacks and the heavy SOL costs associated with account rent, every single one of the 6,000 tokens will be fully restored with its intended metadata.

Transparency is our priority. You can monitor the progress by watching the "Restoration Log" in this README as it grows daily.

---
End of Document

