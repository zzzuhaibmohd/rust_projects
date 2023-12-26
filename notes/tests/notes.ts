import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { assert } from "chai";
import { Notes } from "../target/types/notes";

describe("notes", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();

  anchor.setProvider(provider);

  const program = anchor.workspace.Notes as Program<Notes>;

  let note = anchor.web3.Keypair.generate(); // Create a new account for signer

  it("user can create a note", async () => {
    await program.rpc.createNote("Hello World!!!", {
      accounts: {
        note: note.publicKey,
        user: provider.wallet.publicKey, // The signer of the note account
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [note],
    });

    let newNote = await program.account.note.fetch(note.publicKey);
    assert.strictEqual(newNote.content, "Hello World!!!");
    assert.strictEqual(
      newNote.user.toBase58(),
      provider.wallet.publicKey.toBase58()
    );
  });

  it("user can delete a note", async () => {
    await program.rpc.deleteNote({
      accounts: {
        note: note.publicKey,
        user: provider.wallet.publicKey,
      },
    });

    let deletedNote = await program.account.note.fetchNullable(note.publicKey);
    assert.ok(deletedNote == null);
  });
});
