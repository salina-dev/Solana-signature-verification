import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Airdrop } from "../target/types/airdrop";
import { Keypair, SystemProgram } from "@solana/web3.js";
import * as nacl from "tweetnacl";
import { assert } from "chai";

describe("Airdrop", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Airdrop as Program<Airdrop>;

  console.log("Program ID:", program.programId.toBase58());
  console.log("Provider RPC:", provider.connection.rpcEndpoint);

  const DISTRIBUTOR_SECRET_KEY = new Uint8Array([
    // REPLACE THIS with actual 64-byte secret key for the hardcoded EXPECTED_DISTRIBUTOR pubkey
  ]);
  const distributor = Keypair.fromSecretKey(DISTRIBUTOR_SECRET_KEY);

  it("Verify valid signature", async () => {
    const recipient = Keypair.generate().publicKey;
    const amount = new anchor.BN(1000);

    const message = Buffer.concat([
      recipient.toBuffer(),
      Buffer.from(amount.toArray("le", 8)),
    ]);

    const signature = nacl.sign.detached(message, distributor.secretKey);

    const ed25519Ix = anchor.web3.Ed25519Program.createInstructionWithPublicKey({
      publicKey: distributor.publicKey.toBuffer(),
      message,
      signature,
    });

    try {
      await program.methods
        .verifyAirdropSignature(recipient, amount, Array.from(signature))
        .accounts({
          ed25519InstructionSysvar: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
          systemProgram: SystemProgram.programId,
        })
        .preInstructions([ed25519Ix])
        .rpc();
      assert.ok(true, "Valid signature should succeed");
    } catch (err) {
      console.error("Valid signature error:", err);
      throw err;
    }
  });

  it("Verify invalid signature", async () => {
    const recipient = Keypair.generate().publicKey;
    const amount = new anchor.BN(1000);

    const message = Buffer.concat([
      recipient.toBuffer(),
      Buffer.from(amount.toArray("le", 8)),
    ]);

    const wrongKeypair = Keypair.generate();
    const signature = nacl.sign.detached(message, wrongKeypair.secretKey);

    const ed25519Ix = anchor.web3.Ed25519Program.createInstructionWithPublicKey({
      publicKey: wrongKeypair.publicKey.toBuffer(),
      message,
      signature,
    });

    try {
      await program.methods
        .verifyAirdropSignature(recipient, amount, Array.from(signature))
        .accounts({
          ed25519InstructionSysvar: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
          systemProgram: SystemProgram.programId,
        })
        .preInstructions([ed25519Ix])
        .rpc();
      assert.fail("Expected transaction to fail with InvalidDistributorKey");
    } catch (err) {
      console.log("Invalid signature error:", err.message);
      assert.include(
        err.toString(),
        "Invalid distributor public key",
        "Expected InvalidDistributorKey error"
      );
    }
  });
});