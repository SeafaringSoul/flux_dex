import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { FluxDex } from "../target/types/flux_dex";
import { PublicKey, Keypair } from "@solana/web3.js";
import {
  createMint,
  getAssociatedTokenAddressSync,
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

  let tokenAMint:PublicKey;
  let tokenBMint:PublicKey;
  let poolPda:PublicKey;
  let lpMintPda:PublicKey;
  let tokenAVault:PublicKey;
  let tokenBVault:PublicKey;

  before(async()=>{
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
      [Buffer.from("token_vault_a"),poolPda.toBuffer()],
      program.programId
    );

    [tokenBVault] = PublicKey.findProgramAddressSync(
      [Buffer.from("token_vault_b"),poolPda.toBuffer()],
      program.programId
    );

    console.log("✅ Pool PDA:", poolPda.toBase58());
    console.log("✅ LP Mint PDA:", lpMintPda.toBase58());
    console.log("✅ Token A Vault PDA:", tokenAVault.toBase58());
    console.log("✅ Token B Vault PDA:", tokenBVault.toBase58());

  })

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
});
