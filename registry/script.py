import os

# File paths based on your repository structure
input_file = 'registry/all_mints.txt'
output_dir = 'docs'
output_file = os.path.join(output_dir, 'royalties-enforcement.md')

# Ensure output directory exists
if not os.path.exists(output_dir):
    os.makedirs(output_dir)

# Professional Header and Disclaimer
header = """# Royalties Enforcement

**Important Notice:** These Devnet assets are strictly created for internal testing and development purposes. They possess no intrinsic or monetary value and cannot be exchanged, purchased, or sold using real funds. It won't give airdrop access/eligibility.

### PoW

| Test | Link | Value |
| :--- | :--- | :--- |
"""

try:
    if not os.path.exists(input_file):
        print(f"Error: Could not find {input_file}")
    else:
        with open(input_file, 'r') as f:
            mints = [line.strip() for line in f if line.strip()]

        with open(output_file, 'w') as f:
            f.write(header)
            for i, mint in enumerate(mints, 1):
                # Generate Solscan links with 4-digit formatting (0001, 0002...)
                row = f"| {i:04d} | https://solscan.io/token/{mint}?cluster=devnet | DEVNET |\n"
                f.write(row)
                
        print(f"Success! {len(mints)} addresses processed.")
        print(f"File saved to: {output_file}")

except Exception as e:
    print(f"An error occurred: {e}")
  
