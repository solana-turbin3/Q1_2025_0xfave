import { Connection, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import wallet from "./dev-wallet.json";

// Import keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// Create a Solana devnet connection
const connection = new Connection("https://devnet.helius-rpc.com/?api-key=b47cced4-ed9e-4419-9911-d25e0f108af2");

(async () => {
    try {
        // Claim 2 devnet SOL tokens
        const txhash = await connection.requestAirdrop(
            keypair.publicKey,
            2 * LAMPORTS_PER_SOL
        );

        console.log(
            `Success! Check out your TX here: https://explorer.solana.com/tx/${txhash}?cluster=devnet`
        );
    } catch (e) {
        console.error(`Oops, something went wrong: ${e}`);
    }
})();
