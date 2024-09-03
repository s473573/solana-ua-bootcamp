import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { Favourites } from "../target/types/favourites";
import { airdropIfRequired, getCustomErrorMessage } from "@solana-developers/helpers";
import { expect, describe, test } from '@jest/globals';
import { systemProgramErrors } from "./system-program-errors";

describe("favourites", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Favourites as Program<Favourites>;

  test("Writes our favourites to the blockchain", async () => {
    const user = web3.Keypair.generate();

    console.log(`User public key: ${user.publicKey}`);

    await airdropIfRequired(
      anchor.getProvider().connection,
      user.publicKey,
      0.5 * web3.LAMPORTS_PER_SOL,
      1 * web3.LAMPORTS_PER_SOL
    );

    // Here's what we want to write to the blockchain
    const favouriteNumber = new anchor.BN(23);
    const favouriteColor = "red";

    // Make a transaction to write to the blockchain
    let tx: string | null = null;
    try {
      tx = await program.methods
        // Call the set_favourites instruction handler
        .setFavourites(favouriteNumber, favouriteColor)
        .accounts({
          user: user.publicKey,
          // Note that both `favourites` and `system_program` are added
          // automatically.
        })
        // Sign the transaction
        .signers([user])
        // Send the transaction to the cluster or RPC
        .rpc();
    } catch (thrownObject) {
      // Let's properly log the error, so we can see the program involved
      // and (for well known programs) the full log message.

      console.error("Transaction failed:", thrownObject);

      const rawError = thrownObject as Error;
      throw new Error(getCustomErrorMessage(systemProgramErrors, rawError.message));
    }

    console.log(`Tx signature: ${tx}`);

    // Calculate the PDA account address that holds the user's favourites
    const [favouritesPda, _favouritesBump] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("favourites"), user.publicKey.toBuffer()],
      program.programId
    );

    const accountData = await program.account.favourites.fetch(favouritesPda);

    // And make sure it matches!
    expect(accountData.color).toEqual(favouriteColor);
    expect(accountData.number.toString()).toEqual(favouriteNumber.toString());
  }, 25000 ); // this long-duration test requires high timeout
});
