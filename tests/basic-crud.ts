import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { BasicCrud } from "../target/types/basic_crud";

describe("basic-crud", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.AnchorProvider.local();
  const program = anchor.workspace.BasicCrud as Program<BasicCrud>;
  const wallet = provider.wallet as anchor.Wallet;

    // Add your test here.
    it("Initialize Owner", async () => {
      let [data_account, vPDA1] =
        await anchor.web3.PublicKey.findProgramAddress(
          [Buffer.from("data"),, wallet.publicKey.toBuffer()],
          program.programId
        );
    
    console.log("Hello1");
    const tx = await program.methods.setdata(
      new anchor.BN(123),
      "Hello World"
      )
      .accounts({
        authority: wallet.publicKey,
        dataAccount: data_account,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
      console.log("Your transaction signature", tx);
      console.log("Hello2");


    const data = await program.account.dataAccount.fetch(data_account);
    console.log("Your data", data);

    console.log(data.integerdata.toString());
    console.log(data.stringdata);
    console.log(data.keydata.toString());

  });
});
