import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MerkleRewards } from "../target/types/merkle_rewards";
import * as assert from "assert";
import { sha256 } from "js-sha256";

describe("merkle_rewards", () => {
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.MerkleRewards as Program<MerkleRewards>;

  it("Initializes the program state", async () => {
    const merkleAccount = anchor.web3.Keypair.generate();
    const root = Uint8Array.from(sha256.create().update("test").array());

    await program.rpc.initialize(Array.from(root), {
      accounts: {
        merkleAccount: merkleAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [merkleAccount],
    });

    const account = await program.account.merkleAccount.fetch(merkleAccount.publicKey);
    assert.deepEqual(account.merkleRoot, root);
  });

  it("Claims a reward", async () => {
    // Prepare the proof and leaf (this should match the Merkle tree logic)
    const proof = [
      Uint8Array.from(sha256.create().update("user2:5").array()),
      Uint8Array.from(sha256.create().update("user3:5").array()),
    ];
    const leaf = Uint8Array.from(sha256.create().update("user1:5").array());

    const userAccount = anchor.web3.Keypair.generate();

    await program.rpc.claim(proof.map(p => Array.from(p)), Array.from(leaf), {
      accounts: {
        user: userAccount.publicKey,
        merkleAccount: merkleAccount.publicKey,
      },
    });

    const account = await program.account.user.fetch(userAccount.publicKey);
    assert.ok(account.hasClaimed);
  });
});
