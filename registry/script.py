import os

# File paths based on your current location (inside registry folder)
input_file = 'all_mints.txt'
# Moving one level up to find/create the docs folder
output_dir = os.path.join('..', 'docs')
output_file = os.path.join(output_dir, 'royalties-enforcement.md')

# Ensure output directory exists (creates 'docs' folder outside 'registry')
if not os.path.exists(output_dir):
    os.makedirs(output_dir)

# Professional Header and Disclaimer
header = """# Royalties Enforcement

**Important Notice:** These Devnet assets are strictly created for internal testing and development purposes. They possess no intrinsic or monetary value. It can't be buy/sell using real funds. It won't give airdrop access/eligibility.

### PoW

| Test | Link | Value |
| :--- | :--- | :--- |
"""

try:
    if not os.path.exists(input_file):
        print(f"Error: Could not find {input_file} in the current directory.")
    else:
        with open(input_file, 'r') as f:
            mints = [line.strip() for line in f if line.strip()]

        with open(output_file, 'w') as f:
            f.write(header)
            for i, mint in enumerate(mints, 1):
                # Row format with 4-digit numbering and Devnet Solscan link
                row = f"| {i:04d} | https://solscan.io/token/{mint}?cluster=devnet | DEVNET |\n"
                f.write(row)
                
        print(f"Success! {len(mints)} addresses processed.")
        print(f"File saved to: {output_file}")

except Exception as e:
    print(f"An error occurred: {e}")
    
