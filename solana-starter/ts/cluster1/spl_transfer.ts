import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "../Turbin3-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("A9tqoqRdmBcMbJRMDc3eutrMiu5Ly6amU4uiXrKWffbg");

// Recipient address
const to = new PublicKey("Gh9ZwHsJgRgC6Yqsoyj1W7sUZd5x7zW7i9UeJYn7t9v");

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const fromTokenATA = await getOrCreateAssociatedTokenAccount(connection, keypair, mint, keypair.publicKey);

        // Get the token account of the toWallet address, and if it does not exist, create it
        const toTokenATA = await getOrCreateAssociatedTokenAccount(connection, keypair, mint, to);

        // Transfer the new token to the "toTokenAccount" we just created
        const signature = await transfer(connection, keypair, fromTokenATA.address, toTokenATA.address, keypair, 2e6);

        // 3MHM3zh1HngHYuQT6eLfWsz6Rd2NZawAdhk9ez72rQixLuTCPenej56vM5uvbHpFyR2WRFYqBcz7n8YVKSjwW1ML
        console.log(signature)
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();
