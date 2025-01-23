import {
  Connection,
  Keypair,
  PublicKey,
  Transaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import {
  createMint,
  createAssociatedTokenAccount,
  mintTo,
  getOrCreateAssociatedTokenAccount,
  TOKEN_PROGRAM_ID,
  createFreezeAccountInstruction,
  createThawAccountInstruction,
  transfer,
} from "@solana/spl-token";

// Set up the connection to the local Solana validator
const connection = new Connection("http://localhost:8899", "confirmed");

// Generate keypairs for the dev, user1, user2, and attacker
const dev = Keypair.generate();
const user1 = Keypair.generate();
const user2 = Keypair.generate();
const attacker = Keypair.generate();
const freezeAuthority = Keypair.generate();

// Helper function to airdrop SOL to a wallet
async function airdropSol(wallet: PublicKey, amount: number) {
  const airdropSignature = await connection.requestAirdrop(wallet, amount);
  await connection.confirmTransaction(airdropSignature);
}

// Function to freeze the user's ATA
async function freezeUserAta(
  userAta: PublicKey,
  mint: PublicKey,
  freezeAuthority: Keypair
) {
  const freezeInstruction = createFreezeAccountInstruction(
    userAta,
    mint,
    freezeAuthority.publicKey,
    [],
    TOKEN_PROGRAM_ID
  );

  const transaction = new Transaction().add(freezeInstruction);
  await sendAndConfirmTransaction(connection, transaction, [freezeAuthority]);
  console.log("User's ATA frozen.");
}

// Function to thaw the user's ATA
async function thawUserAta(
  userAta: PublicKey,
  mint: PublicKey,
  freezeAuthority: Keypair
) {
  const thawInstruction = createThawAccountInstruction(
    userAta,
    mint,
    freezeAuthority.publicKey,
    [],
    TOKEN_PROGRAM_ID
  );

  const transaction = new Transaction().add(thawInstruction);
  await sendAndConfirmTransaction(connection, transaction, [freezeAuthority]);
  console.log("User's ATA thawed.");
}

async function main() {
  // Step 1: Airdrop SOL to the dev, user1, user2, attacker, and freeze authority
  console.log("Airdropping SOL to dev, user1, user2, attacker, and freeze authority...");
  await airdropSol(dev.publicKey, 1e9); // 1 SOL
  await airdropSol(user1.publicKey, 1e9); // 1 SOL
  await airdropSol(user2.publicKey, 1e9); // 1 SOL
  await airdropSol(attacker.publicKey, 1e9); // 1 SOL
  await airdropSol(freezeAuthority.publicKey, 1e9); // 1 SOL

  // Step 2: Create a new mint with a freeze authority
  console.log("Creating a new mint...");
  const mint = await createMint(
    connection,
    dev, // Payer
    dev.publicKey, // Mint authority
    freezeAuthority.publicKey, // Freeze authority
    9 // Decimals
  );
  console.log("Mint created:", mint.toBase58());

  // Step 3: Create ATAs for user1, user2, and attacker
  console.log("Creating ATAs for user1, user2, and attacker...");
  const user1Ata = await getOrCreateAssociatedTokenAccount(
    connection,
    dev,
    mint,
    user1.publicKey
  );
  const user2Ata = await getOrCreateAssociatedTokenAccount(
    connection,
    dev,
    mint,
    user2.publicKey
  );
  const attackerAta = await getOrCreateAssociatedTokenAccount(
    connection,
    dev,
    mint,
    attacker.publicKey
  );
  console.log("User1's ATA created:", user1Ata.address.toBase58());
  console.log("User2's ATA created:", user2Ata.address.toBase58());
  console.log("Attacker's ATA created:", attackerAta.address.toBase58());

  // Step 4: Dev mints tokens to user1's ATA
  console.log("Dev minting tokens to user1's ATA...");
  await mintTo(
    connection,
    dev,
    mint,
    user1Ata.address,
    dev.publicKey,
    100e9 // 100 tokens (with 9 decimals)
  );
  console.log("Tokens minted to user1's ATA.");

  // Step 5: Freeze user1's ATA
  console.log("Freezing user1's ATA...");
  await freezeUserAta(user1Ata.address, mint, freezeAuthority);

  // Step 6: User1 tries to transfer tokens to User2 (should fail)
  console.log("User1 attempting to transfer tokens to User2...");
  try {
    await transfer(
      connection,
      user1,
      user1Ata.address,
      user2Ata.address,
      user1.publicKey,
      10e9 // 10 tokens (with 9 decimals)
    );
    console.log("Transfer succeeded (this should not happen).");
  } catch (error) {
    if (error instanceof Error) {
      if (error instanceof Error) {
        if (error instanceof Error) {
          if (error instanceof Error) {
            if (error instanceof Error) {
              if (error instanceof Error) {
                console.log("Transfer failed (expected):", error.message);
              } else {
                console.log("Transfer failed (expected):", error);
              }
            } else {
              console.log("Transfer failed (expected):", error);
            }
          } else {
            console.log("Transfer failed (expected):", error);
          }
        } else {
          console.log("Transfer failed (expected):", error);
        }
      } else {
        console.log("Transfer failed (expected):", error);
      }
    } else {
      console.log("Transfer failed (expected):", error);
    }
  }

  // Step 7: Attacker tries to send tokens to User1 (should fail)
  console.log("Attacker attempting to send tokens to User1...");
  try {
    await transfer(
      connection,
      attacker,
      attackerAta.address,
      user1Ata.address,
      attacker.publicKey,
      10e9 // 10 tokens (with 9 decimals)
    );
    console.log("Transfer succeeded (this should not happen).");
  } catch (error) {
    console.log("Transfer failed (expected):", error instanceof Error ? error.message : String(error));
  }

  // Step 8: Thaw user1's ATA
  console.log("Thawing user1's ATA...");
  await thawUserAta(user1Ata.address, mint, freezeAuthority);

  // Step 9: User1 transfers tokens to User2 (should now succeed)
  console.log("User1 attempting to transfer tokens to User2 again...");
  await transfer(
    connection,
    user1,
    user1Ata.address,
    user2Ata.address,
    user1.publicKey,
    10e9 // 10 tokens (with 9 decimals)
  );
  console.log("Transfer succeeded.");
}
main().catch(console.error);
