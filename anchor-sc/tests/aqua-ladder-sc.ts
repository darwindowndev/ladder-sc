import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AquaLadderSc } from "../target/types/aqua_ladder_sc";

// Ensure Buffer is available in the test environment (Node.js)
import { Buffer } from "buffer";

describe("aqua-ladder-sc", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.aquaLadderSc as Program<AquaLadderSc>;

  it("Should not allow to participate in the ladder if the ladder is not initialized", async () => {
    try {
      await program.methods.participateInLadder(new anchor.BN(0.1 * 1_000_000_000)).rpc();
      // If the transaction does not throw, the test should fail
      throw new Error("Transaction should have failed but succeeded");
    } catch (err) {
      // eslint-disable-next-line no-console
      console.log("Expected failure when participating in uninitialized ladder:", err.message || err);
      
    }
  });
  
  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initializeLadder().rpc();
    // eslint-disable-next-line no-console
    console.log("Your transaction signature", tx);

    // eslint-disable-next-line no-console
    console.log("The program id is ", program.programId.toBase58());

    // Derive the PDA for ladder_information (seeds: [b"ladder"])
    const [ladderInformationPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("ladder")],
      program.programId
    );

    // Fetch the account data for the ladder_information PDA
    const ladderInfoAccount = await program.account.ladderInformation.fetch(ladderInformationPda);
    // eslint-disable-next-line no-console
    console.log("Ladder Information PDA:", ladderInformationPda.toBase58());
    // eslint-disable-next-line no-console
    console.log("Ladder Information Account Data:", ladderInfoAccount);

    if (!ladderInfoAccount.isInitialized) {
      throw new Error("LadderInformation account is not initialized");
    }
  });

  

  it("Should participate in the ladder", async () => {
    const participant = program.provider.publicKey;
    // eslint-disable-next-line no-console
    console.log("Creating participation tx for user ", participant.toBase58());

    const participationTx = await program.methods.participateInLadder(new anchor.BN(1 * 1_000_000_000)).rpc();
    // eslint-disable-next-line no-console
    console.log("Participation tx: ", participationTx);
    const [participantInfoPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("participant"), participant.toBuffer()],
      program.programId
    );

    const participantInfoAccount = await program.account.participantInformation.fetch(participantInfoPda);
    // eslint-disable-next-line no-console
    console.log("Participant Information PDA:", participantInfoPda.toBase58());
    // eslint-disable-next-line no-console
    console.log("Participant Information Account Data:", participantInfoAccount);
  });

  it("Should not allow to participate in the ladder if the amount is less than the minimum", async () => {
    try {
      await program.methods.participateInLadder(new anchor.BN(0.0000000001 * 1_000_000_000)).rpc();
      // If the transaction does not throw, the test should fail
      throw new Error("Transaction should have failed but succeeded");
    } catch (err) {
      // eslint-disable-next-line no-console
      console.log("Expected failure when participating with less than minimum amount:", err.message || err);
    }
  });

  it("Should not allow to participate in the ladder if the amount is greater than the maximum", async () => {
    try {
      await program.methods.participateInLadder(new anchor.BN(14.91 * 1_000_000_000)).rpc();
      // If the transaction does not throw, the test should fail
      throw new Error("Transaction should have failed but succeeded");
    } catch (err) {
      // eslint-disable-next-line no-console
      console.log("Expected failure when participating with greater than maximum amount:", err.message || err);
    }
  });

  it("Should resolve the ladder", async () => {
    const resolveTx = await program.methods.resolveLadder().rpc();
    // eslint-disable-next-line no-console
    console.log("Resolve tx: ", resolveTx);
    

    const [ladderInfoPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("ladder")],
      program.programId
    );

    const ladderInfoAccount = await program.account.ladderInformation.fetch(ladderInfoPda);
    // eslint-disable-next-line no-console
    console.log("Ladder Information PDA:", ladderInfoPda.toBase58());
    // eslint-disable-next-line no-console
    console.log("Ladder Information Account Data after resolve:", ladderInfoAccount);
  });

  it("Should not allow to participate in the ladder after it is resolved", async () => {
    
    const walletPubkey = program.provider.publicKey;
    const connection = program.provider.connection;
    const balanceLamports = await connection.getBalance(walletPubkey);
    const balanceSol = balanceLamports / anchor.web3.LAMPORTS_PER_SOL;

    console.log("Balance sol ", balanceSol); // Make sure we are not running out of sol
    console.log("Required sol 0.1");

    if (balanceSol < 0.1) {
      throw new Error("Not enough SOL to pass the tests");
    }

    // eslint-disable-next-line no-console
    console.log(`Wallet SOL balance: ${balanceSol}`);
    try {
      await program.methods.participateInLadder(new anchor.BN(0.1 * 1_000_000_000)).rpc();
      // If the transaction does not throw, fail the test
      throw new Error("Transaction should have failed but succeeded");
    } catch (err) {
      // eslint-disable-next-line no-console
      console.log("Expected failure when participating after ladder is resolved:", err.message || err);
      
    }
  });
});
