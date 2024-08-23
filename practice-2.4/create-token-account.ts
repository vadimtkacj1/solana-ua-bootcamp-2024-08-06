import "dotenv/config";
import { getExplorerLink } from "@solana-developers/helpers";
import {
	  Connection,
	    Keypair,
	      PublicKey,
	        clusterApiUrl,
} from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount } from "@solana/spl-token";

let privateKey = process.env["SECRET_KEY"];
if (privateKey === undefined) {
	  console.log("Add SECRET_KEY to .env!");
	    process.exit(1);
}
const asArray = Uint8Array.from(JSON.parse(privateKey));
const sender = Keypair.fromSecretKey(asArray);

const connection = new Connection(clusterApiUrl("devnet"));

console.log(
	  `🔑 Our pubic key is: ${sender.publicKey.toBase58()}`
);

const tokenMintAccount = new PublicKey(
	  "DAk5ckdJBhkB93Q4mhkRF3oMLkpZT4Kag3WQksftzFXt"
);
const recipient = new PublicKey("GjfgDFKcciktu7M3piLesuBzUfGeTcJvb6eDozDp3H7s");

const tokenAccount = await getOrCreateAssociatedTokenAccount(
	  connection,
	    sender,
	      tokenMintAccount,
	        recipient
);

console.log(`Token Account: ${tokenAccount.address.toBase58()}`);

const link = getExplorerLink(
	  "address",
	    tokenAccount.address.toBase58(),
	      "devnet"
);

console.log(`✅ Created token account: ${link}`);
