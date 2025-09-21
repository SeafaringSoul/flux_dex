import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { FluxDex } from "../target/types/flux_dex";
import { PublicKey, Keypair } from "@solana/web3.js";
import {
  createAssociatedTokenAccount,
  createMint,
  getAccount,
  getAssociatedTokenAddressSync,
  mintTo,
  Token,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";

describe("flux_dex", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.fluxDex as Program<FluxDex>;
  const authority = provider.wallet.publicKey;

  let tokenAMint: PublicKey;
  let tokenBMint: PublicKey;
  let poolPda: PublicKey;
  let lpMintPda: PublicKey;
  let tokenAVault: PublicKey;
  let tokenBVault: PublicKey;
  let userTokenA: PublicKey;
  let userTokenB: PublicKey;
  let userLpToken: PublicKey;

  before(async () => {
    // create mint
    tokenAMint = await createMint(
      provider.connection,
      (provider.wallet as anchor.Wallet).payer, // Keypair
      authority,
      null,
      6
    );
    tokenBMint = await createMint(
      provider.connection,
      (provider.wallet as anchor.Wallet).payer, // Keypair
      authority,
      null,
      6
    );

    console.log("✅ Token A Mint:", tokenAMint.toBase58());
    console.log("✅ Token B Mint:", tokenBMint.toBase58());

    [poolPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("pool"), authority.toBuffer()],
      program.programId
    );

    // derive lp_mint PDA
    [lpMintPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("lp_mint"), poolPda.toBuffer()],
      program.programId
    );

    [tokenAVault] = PublicKey.findProgramAddressSync(
      [Buffer.from("token_vault_a"), poolPda.toBuffer()],
      program.programId
    );

    [tokenBVault] = PublicKey.findProgramAddressSync(
      [Buffer.from("token_vault_b"), poolPda.toBuffer()],
      program.programId
    );

    userTokenA = await createAssociatedTokenAccount(
      provider.connection,
      (provider.wallet as anchor.Wallet).payer,
      tokenAMint,
      authority
    );
    userTokenB = await createAssociatedTokenAccount(
      provider.connection,
      (provider.wallet as anchor.Wallet).payer,
      tokenBMint,
      authority
    );
    userLpToken = await getAssociatedTokenAddressSync(lpMintPda, authority);

    // 给用户mint 初始代币
    await mintTo(
      provider.connection,
      (provider.wallet as anchor.Wallet).payer,
      tokenAMint,
      userTokenA,
      authority,
      1_000_000_000
    );
    await mintTo(
      provider.connection,
      (provider.wallet as anchor.Wallet).payer,
      tokenBMint,
      userTokenB,
      authority,
      1_000_000_000 // 1000 tokenB
    );

    console.log("✅ Setup done.");
    console.log("✅ poolpda address:",poolPda.toBase58())
  });

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initializePool(0.3)
      .accounts({
        authority: authority,
        pool: poolPda,
        tokenAMint: tokenAMint,
        tokenBMint: tokenBMint,
        tokenAVault: tokenAVault,
        tokenBVault: tokenBVault,
        lpMint: lpMintPda,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("Add liquidity", async () => {

    const [positionPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("position"), authority.toBuffer(), poolPda.toBuffer()],
    program.programId
  );

    const tx = await program.methods
      .addLiquidity(
        new anchor.BN(100_000), // desired A
        new anchor.BN(50_000), // desired B
        new anchor.BN(90_000), // min A
        new anchor.BN(40_000), // min B
        new anchor.BN(0) // min LP
      )
      .accounts({
        user: authority,
        pool: poolPda,
        userTokenA: userTokenA,
        userTokenB: userTokenB,
        userLpToken: userLpToken,
        poolTokenAVault: tokenAVault,
        poolTokenBVault: tokenBVault,
        lpMint: lpMintPda,
        postion: positionPda,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    console.log("✅ Liquidity added, tx:", tx);

    // check结果
    const userAAcc = await getAccount(provider.connection, userTokenA);
    const userBAcc = await getAccount(provider.connection, userTokenB);
    const userLpAcc = await getAccount(provider.connection, userLpToken);

    console.log("User A balance:", Number(userAAcc.amount));
    console.log("User B balance:", Number(userBAcc.amount));
    console.log("User LP balance:", Number(userLpAcc.amount));
  });
});
