import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
// import { TOKEN_PROGRAM_ID, Token } from '@solana/spl-token';
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import { Quadratic } from '../target/types/quadratic';

describe('quadratic', async () => {

  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Quadratic as Program<Quadratic>;

  // let poolAcc = anchor.web3.Keypair.generate();
  // let tokenAuthority = anchor.web3.Keypair.generate(); 
  // let authority = anchor.web3.Keypair.generate();
  let project = anchor.web3.Keypair.generate();
  // let projectPerson = anchor.web3.Keypair.generate();
  // let funder = anchor.web3.Keypair.generate();

  // let mintAmount = new anchor.BN(1000000);

  // let token = await Token.createMint(
  //   provider.connection,
  //   provider.wallet.publicKey,
  //   tokenAuthority.publicKey,
  //   null,
  //   0,
  //   TOKEN_PROGRAM_ID
  // );

  // let initializerTokenAccountA = await token.createAccount(
  //   provider.wallet.publicKey
  // );
  // await token.mintTo(
  //   initializerTokenAccountA,
  //   tokenAuthority.publicKey,
  //   [tokenAuthority],
  //   mintAmount
  // )

  // let poolAmountToken = new anchor.BN(100);

  it('create match Pool!', async () => {
    console.log("Creating Match Pool")
    // Add your test here.
    // const tx = await program.rpc.createMatchPool(new anchor.BN(8), poolAmountToken, {
    //   accounts: {
    //     poolCreator: authority.publicKey,
    //     pool: initializerTokenAccountA,
    //     poolAmountToken: poolAmountToken,
    //     poolAccount: poolAcc,
    //     systemProgram: anchor.web3.SystemProgram.programId,
    //     tokenProgram: Token.TOKEN_PROGRAM_ID,
    //   }
    // });
    // console.log("match pool transaction signature::=>", tx);
  });

  it('create project!', async () => {
    // Add your test here.
    const tx = await program.rpc.createProject({
      accounts: {
        project: project.publicKey,
      }
    });
    console.log("project created signature::=>", tx);
  });
});
