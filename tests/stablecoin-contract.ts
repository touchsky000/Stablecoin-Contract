import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { StablecoinContract } from "../target/types/stablecoin_contract";
import {
  PublicKey,
  Transaction,
  ComputeBudgetProgram,
  SystemProgram,
  Keypair,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID, 
  getAssociatedTokenAddressSync, 
  getOrCreateAssociatedTokenAccount, 
  Token} from "@solana/spl-token"
describe("stablecoin-contract", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.StablecoinContract as Program<StablecoinContract>;
  let admin = Keypair.generate()
  const user = Keypair.generate()
  let globalState = Keypair.generate()
  let stablecoinMint = Keypair.generate();
  const mintCap = 100_000
  const reserveRatio = 50

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Initialize Global State", async () => {
    // Request airdrop for admin
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(admin.publicKey, anchor.web3.LAMPORTS_PER_SOL),
    );

    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(user.publicKey, anchor.web3.LAMPORTS_PER_SOL)
    );

    console.log("Global State Public Key: ", globalState.publicKey.toBase58());

    // Check if the global state account already exists
    const globalStateAccount = await provider.connection.getAccountInfo(globalState.publicKey);
    if (globalStateAccount) {
      console.log("Global state account already exists:", globalStateAccount);
    } else {
      console.log("Global state account is not initialized. Proceeding to initialize.");
    }

    // Initialize global state
    const tx = await program.methods.initializeGlobalState(
      new anchor.BN(mintCap),
      new anchor.BN(reserveRatio)
    ).accounts({
      globalState: globalState.publicKey,
      admin: admin.publicKey,
      systemProgram: SystemProgram.programId,
    }).signers([admin, globalState])
      .rpc();

  });

  it("update mint cap", async () => {
    const new_cap = 200_000
    const tx = await program.methods.updateMintCap(new anchor.BN(new_cap))
      .accounts({
        globalState: globalState.publicKey,
        admin: admin.publicKey
      }).signers([admin])
      .rpc()
  })

  it("update_reserve_ratio", async () => {
    const new_ratio = 40
    const tx = await program.methods.updateReserveRatio(new anchor.BN(new_ratio))
      .accounts({
        globalState: globalState.publicKey,
        admin: admin.publicKey
      }).signers([admin])
      .rpc()
  })

  it("emergency_pause", async () => {
    const status = true
    const tx = await program.methods.emergencyPause(status)
    .accounts({
      globalState: globalState.publicKey,
      admin: admin.publicKey
    }).signers([admin])
    .rpc()
  })


  it("admin mint stablecoin", async () => {
    let mint, userTokenAccount, adminTokenAccount, vaultTokenAccount;
    stablecoinMint = await Token.createMint(
      provider.connection,
      admin,
      admin.publicKey,
      null,
      6, // Assuming 6 decimals for the stablecoin
      TOKEN_PROGRAM_ID
    );

  });
  
  
  
  

  // it("deposite_collateral", async () => {
  //   const amount = 100
  //   const tx = await program.methods.deposite_collateral(new anchor.BN(amount))
  //   .accounts({
  //     user: admin.publicKey,
  //     userTokenAccount: 

  //     globalState: globalState.publicKey,

  //   }).signers([admin])
  // })
});
