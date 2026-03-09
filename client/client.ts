import * as anchor from "@coral-xyz/anchor";

async function main() {

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  console.log("Conectado a Solana");

  const program = anchor.workspace.TiendaVideojuegos;

  console.log("Programa cargado:", program.programId.toString());

}

main().catch(console.error);
