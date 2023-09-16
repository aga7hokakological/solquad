import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Solquad } from "../target/types/solquad";
import { utf8 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import {BN} from "bn.js";

describe("solquad", async () => {
  // Configure the client to use the local cluster.
  const provider = anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Solquad as Program<Solquad>;

  const escrowOwner = anchor.web3.Keypair.generate();
  const projectOwner1 = anchor.web3.Keypair.generate();
  const projectOwner2 = anchor.web3.Keypair.generate();
  const projectOwner3 = anchor.web3.Keypair.generate();
  const voter1 = anchor.web3.Keypair.generate();
  const voter2 = anchor.web3.Keypair.generate();
  const voter3 = anchor.web3.Keypair.generate();
  const voter4 = anchor.web3.Keypair.generate();
  const voter5 = anchor.web3.Keypair.generate();
  const voter6 = anchor.web3.Keypair.generate();

  const [escrowPDA] = await anchor.web3.PublicKey.findProgramAddressSync([
    utf8.encode("escrow"),
    escrowOwner.publicKey.toBuffer(),
  ],
    program.programId
  );

  const [poolPDA] = await anchor.web3.PublicKey.findProgramAddressSync([
    utf8.encode("pool"),
    escrowOwner.publicKey.toBuffer(),
  ],
    program.programId
  );

  it("Is initialized!", async () => {

    console.log("ESCROW PDA", escrowPDA);
    // Add your test here.
    const tx = await program.methods.initializeEscrow(new BN(10000)).accounts({
      escrowAccount: escrowPDA,
      escrowSigner: escrowOwner.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Initialize pool", async () => {

    console.log("POOL PDA", poolPDA);
    const tx = await program.methods.initializePool().accounts({
      poolAccount: poolPDA,
      poolSigner: escrowOwner.publicKey,
    })
  })

  it("initialize project", async () => {
    const tx1 = await program.methods.initializeProject("").accounts({
      projectAccount: program.provider.publicKey,
      projectOwner: escrowOwner.publicKey,
    })
  })

  it("add projects to pool", async () => {
    

    const tx2 = await program.methods.addProjectToPool("project1").accounts({
      escrowAccount: escrowPDA,
      poolAccount: poolPDA,
      projectAccount: program.provider.publicKey,
      projectOwner: projectOwner1.publicKey,
    }).signers([projectOwner1]).rpc()
  })
});
