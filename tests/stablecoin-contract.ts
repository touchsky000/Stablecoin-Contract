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
  createMint,
  mintTo,
  getMint,
  getAssociatedTokenAddress,
  getAccount
} from "@solana/spl-token"
describe("stablecoin-contract", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);

  const program = anchor.workspace.StablecoinContract as Program<StablecoinContract>;
  let admin = Keypair.generate()
  const user = Keypair.generate()
  let globalState = Keypair.generate()
  const mintCap = 100_000
  const reserveRatio = 50
  let userTokenAccountAta, vaultTokenAccount, stablecoinMint, collateralMint, stableAta, adminStableAta;

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

    // Check if the global state account already exists
    const globalStateAccount = await provider.connection.getAccountInfo(globalState.publicKey);

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

  it("collate token create", async () => {
    collateralMint = await createMint(
      provider.connection,
      admin,
      admin.publicKey,
      null,
      6, // Assuming 6 decimals for the stablecoin
    );

    userTokenAccountAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      admin,
      collateralMint,
      admin.publicKey
    );
  })

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

  it("emergency_resume", async () => {
    const status = false
    const tx = await program.methods.emergencyPause(status)
      .accounts({
        globalState: globalState.publicKey,
        admin: admin.publicKey
      }).signers([admin])
      .rpc()
  })

  it("deposite_collateral", async () => {
    const amount = new anchor.BN(100)

    const [vaultAuthority] = await PublicKey.findProgramAddressSync(
      [Buffer.from("vault")],
      program.programId
    );

    vaultTokenAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      admin,
      collateralMint,
      vaultAuthority,
      true
    )

    // Mint some tokens to the user's account first
    await mintTo(
      provider.connection,
      admin,
      collateralMint,
      userTokenAccountAta.address,
      admin,
      1000000, // Mint 1 token (with 6 decimals),
      undefined,
      undefined,
      TOKEN_PROGRAM_ID
    );

    const tx = await program.methods.depositCollateral(amount)
      .accounts({
        user: admin.publicKey,
        userTokenAccount: userTokenAccountAta.address,
        vaultTokenAccount: vaultTokenAccount.address,
        globalState: globalState.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID
      })
      .signers([admin])
      .rpc()

  })


  it("stable coin create", async () => {
    const [mintAuthPda] = await PublicKey.findProgramAddressSync(
      [Buffer.from("mint"), globalState.publicKey.toBuffer()],
      program.programId
    );

    stablecoinMint = await createMint(
      provider.connection,
      admin,
      mintAuthPda,
      null,
      6, // Assuming 6 decimals for the stablecoin
    );

    stableAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      admin,
      stablecoinMint,
      mintAuthPda,
      true
    );

  })

  it("admin mint stablecoin", async () => {
    const [mintAuthPda] = await PublicKey.findProgramAddressSync(
      [Buffer.from("mint"), globalState.publicKey.toBuffer()],
      program.programId
    );

    const mintAmount = new anchor.BN(100);

    adminStableAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      admin,
      stablecoinMint,
      admin.publicKey,
      true
    );

    const tx = await program.methods.adminMintStablecoin(mintAmount)
      .accounts({
        user: admin.publicKey,
        stablecoinMint: stablecoinMint,
        userStablecoinAccount: adminStableAta.address,
        mintAuthority: mintAuthPda,
        globalState: globalState.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        mintAuthorityBump: mintAuthPda
      })
      .signers([admin])
      .rpc();
  });

  it("StableCoin balance", async () => {
    const userAta = await getAssociatedTokenAddress(stablecoinMint, admin.publicKey);
    const tokenAccount = await getAccount(provider.connection, userAta);
    console.log(">> Stable balance >>", Number(tokenAccount.amount))
  })

  it("admin redeem", async () => {
    const redeem_amount = new anchor.BN(40)

    const [vaultAuthority] = await PublicKey.findProgramAddressSync(
      [Buffer.from("vault")],
      program.programId
    );

    const adminStableAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      admin,
      stablecoinMint,
      admin.publicKey,
      true
    );

    const tx = await program.methods.adminRedeem(redeem_amount)
      .accounts({
        admin: admin.publicKey,
        stablecoinMint: stablecoinMint,
        adminStablecoinAccount: adminStableAta.address,
        vaultTokenAccount: vaultTokenAccount.address,
        adminCollateralAccount: userTokenAccountAta.address,
        vaultAuthority: vaultAuthority,
        globalState: globalState.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID
      }).signers([admin])
      .rpc()

    console.log("Tx => ", tx)
  })


  it("token balance", async () => {
    const colAta = await getAssociatedTokenAddress(collateralMint, admin.publicKey);
    const tokenAccount = await getAccount(provider.connection, colAta);
    console.log(">> Stable balance >>", Number(tokenAccount.amount))
  })

  it("get Global", async () => {
    const globalStateAccount = await program.account.globalState.fetch(globalState.publicKey);
    console.log("Mint Cap: ", globalStateAccount.mintCap.toString());
    console.log("Total Minted: ", globalStateAccount.totalMinted.toString());
    console.log("Total totalCollateral: ", globalStateAccount.totalCollateral.toString());
  })
});
