import "dotenv/config";
import {
	  Connection,
	    LAMPORTS_PER_SOL,
	      PublicKey,
	        clusterApiUrl,
} from "@solana/web3.js";

const connection = new Connection(clusterApiUrl("devnet"));
console.log(`⚡️ Connected to devnet`);

const publicKey = new PublicKey("Am5WQcqYhEac75aQPHUvscpexjyVG4e2ZgYpJGLrTZkG");

const balanceInLamports = await connection.getBalance(publicKey);

const balanceInSOL = balanceInLamports / LAMPORTS_PER_SOL;

console.log(
	  `💰 The balance for the wallet at address ${publicKey} is: ${balanceInSOL}`
);


