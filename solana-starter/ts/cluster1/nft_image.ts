import wallet from "../Turbin3-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { readFile } from "fs/promises"
import path from 'path';

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        //1. Load image
        //2. Convert image to generic file.
        //3. Upload image

        // const image = await readFile("./nft.png");
        const imagePath = path.join(__dirname, 'nft.png');
        const image = await readFile(imagePath);
        const newIMGFile = createGenericFile(image, "boring.jpg", {
            displayName: "Rug",
            contentType: "image/jpeg",
        });

        const [myUri] = await umi.uploader.upload([newIMGFile]);
        console.log("Your image URI: ", myUri); //Your image URI:  https://arweave.net/6TsRqH6d2dx4sNY6zuvgMiE4Dj6wEQDeKvXBjMDRNeZk
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
