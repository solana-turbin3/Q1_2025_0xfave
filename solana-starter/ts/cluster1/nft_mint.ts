import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, signerIdentity, generateSigner, percentAmount } from "@metaplex-foundation/umi"
import { createNft, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";

import wallet from "../Turbin3-wallet.json"
import base58 from "bs58";

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata())

const mint = generateSigner(umi);

// Succesfully Minted! Check out your TX here:
// https://explorer.solana.com/tx/2k3KdhL1LGSZhpTpE5e5XZgpgM3Cm6WKs7dYHhe8H4itxdV9snGxknUtiUaTBoSFBNXbUGdJA2F5ijKLyEze7QMg?cluster=devnet
// Mint Address:  FMSURpfVrFCvx6k4WTueDt9A6Jjd5P8NCmjUcgHePp1p
// https://explorer.solana.com/address/FMSURpfVrFCvx6k4WTueDt9A6Jjd5P8NCmjUcgHePp1p?cluster=devnet
(async () => {
    let tx = createNft(umi, {
        mint,
        name: "Ckonda",
        uri: "https://crimson-advisory-condor-286.mypinata.cloud/ipfs/bafkreiepvn5kqjoe6p2fqbfpfkhc6g77sytcn4hj7r7sb2zpuo43uzbup4",
        sellerFeeBasisPoints: percentAmount(45),
    })
    let result = await tx.sendAndConfirm(umi);
    const signature = base58.encode(result.signature);
    
    console.log(`Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`)

    console.log("Mint Address: ", mint.publicKey);
})();
