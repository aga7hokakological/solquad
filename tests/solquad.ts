import { join } from "path";
import { readFileSync } from "fs";
import { 
  TOKEN_PROGRAM_ID, 
  LAMPORTS_PER_SOL, 
  createMint, 
  createAccount, 
  MINT_SIZE, 
  mintTo, 
  getMint, 
  createAssociatedTokenAccount, 
  mintToChecked 
} from "@solana/spl-token";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Solquad } from "../target/types/solquad";
import { utf8 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import {BN} from "bn.js";

describe("solquad", async () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Solquad as Program<Solquad>;

  const WALLET_PATH = join(process.env["HOME"]!, ".config/solana/id.json");
  const admin = anchor.web3.Keypair.fromSecretKey(
    Buffer.from(JSON.parse(readFileSync(WALLET_PATH, { encoding: "utf-8" })))
  );

  const AIRDROP_AMOUNT = 5 * LAMPORTS_PER_SOL; // 5 SOL 

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

  airdrop(projectOwner1, provider);

  const [escrowPDA] = await anchor.web3.PublicKey.findProgramAddressSync([
    utf8.encode("escrow"),
    admin.publicKey.toBuffer(),
  ],
    program.programId
  );

  const [poolPDA] = await anchor.web3.PublicKey.findProgramAddressSync([
    utf8.encode("pool"),
    admin.publicKey.toBuffer(),
  ],
    program.programId
  );

  const [projectPDA1] = await anchor.web3.PublicKey.findProgramAddressSync([
    utf8.encode("project"),
    admin.publicKey.toBuffer(),
  ],
    program.programId
  );

  it("Is initialized!", async () => {
    console.log("ESCROW PDA", escrowPDA);
    // Add your test here.
    const escrowTx = await program.methods.initializeEscrow(new BN(10000)).accounts({
      escrowAccount: escrowPDA,
      escrowSigner: admin.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([admin]).rpc();
    console.log("Your transaction signature", escrowTx);

    console.log("POOL PDA", poolPDA);
    const poolTx = await program.methods.initializePool().accounts({
      poolAccount: poolPDA,
      poolSigner: admin.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([admin]).rpc();

    const project1Tx = await program.methods.initializeProject("").accounts({
      projectAccount: projectPDA1,
      projectOwner: admin.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([admin]).rpc();

    const tx1 = await program.methods.addProjectToPool("project1").accounts({
      escrowAccount: escrowPDA,
      poolAccount: poolPDA,
      projectAccount: projectPDA1,
      projectOwner: admin.publicKey,
    }).signers([admin]).rpc();

    const data = await program.account.pool.fetch(
      poolPDA
    )
    console.log("data projects", data.totalProjects)
    console.assert(data.totalProjects == 1);

    const voteTx = await program.methods.voteForProject(admin.publicKey, new BN(10)).accounts({
      poolAccount: poolPDA,
      projectAccount: projectPDA1,
      voterSig: admin.publicKey
    }).signers([]).rpc();

    const project = await program.account.project.fetch(
      projectPDA1
    )
    console.log("amount", project.voterAmount.toString());

    const distribTx = await program.methods.distributeEscrowAmount().accounts({
      escrowAccount: escrowPDA,
      poolAccount: poolPDA,
      projectAccount: projectPDA1,
      escrowOwner: admin.publicKey,
    }).signers([admin]).rpc();

    const ant = await program.account.project.fetch(
      projectPDA1
    )
    console.log("amount", ant.distributedAmt.toString());
  });

  it("Seems like I have less votes but I can maximize my rewards", async () => {

  })
});


async function airdrop(user, provider) {
  const AIRDROP_AMOUNT = 5 * LAMPORTS_PER_SOL; // 5 SOL

  // airdrop to user
  const airdropSignature = await provider.connection.requestAirdrop(
    user.publicKey,
    AIRDROP_AMOUNT
  );
  const { blockhash, lastValidBlockHeight } = await provider.connection.getLatestBlockhash();
  
  await provider.connection.confirmTransaction({
    blockhash: blockhash,
    lastValidBlockHeight: lastValidBlockHeight,
    signature: airdropSignature,
  });

  console.log(`Tx Complete: https://explorer.solana.com/tx/${airdropSignature}?cluster=Localnet`)
}