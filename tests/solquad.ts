import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Solquad } from "../target/types/solquad";

describe("solquad", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Solquad as Program<Solquad>;

  // it("Is initialized!", async () => {
  //   // Add your test here.
  //   const tx = await program.rpc.initialize({});
  //   console.log("Your transaction signature", tx);
  // });

  // it("Create Pool", async () => {
  //   await program.create_pool(new anchor.BN(100000), {
  //     accounts: {

  //     }
  //   })
  // })
});
