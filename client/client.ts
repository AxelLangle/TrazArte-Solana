console.log("Iniciando pruebas del contrato TrazArte...\n");

// 1. Definir el ID de la pieza (como es u64 en Rust, en TypeScript usamos la clase BN - Big Number)
const idPieza = new anchor.BN(1);

// 2. Derivar la dirección PDA exacta de la artesanía usando nuestras semillas
const [piezaPda] = anchor.web3.PublicKey.findProgramAddressSync(
  [
    Buffer.from("artesania"),
    pg.wallet.publicKey.toBuffer(),
    idPieza.toArrayLike(Buffer, "le", 8), // 8 bytes en formato little-endian para el u64
  ],
  pg.program.programId
);

console.log("📍 PDA derivada para la pieza:", piezaPda.toBase58());

// Esta función anónima nos permite usar await de forma ordenada
(async () => {
  // ==========================================
  // CREATE: Registrar la pieza inicial
  // ==========================================
  try {
    console.log("\n--- 1. CREANDO REGISTRO ---");
    const txCreate = await pg.program.methods
      .registrarPieza(idPieza, "Cerámica de barro", "Zumpango de Ocampo")
      .accounts({
        pieza: piezaPda,
        artesano: pg.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    console.log("✅ Transacción de creación exitosa. Hash:", txCreate);
  } catch (e) {
    console.error("❌ Error al crear:", e.message);
  }

  // ==========================================
  // READ: Leer los datos desde la blockchain
  // ==========================================
  try {
    console.log("\n--- 2. LEYENDO DATOS ---");
    // Usamos el nombre del struct en minúsculas (artesania)
    const piezaData = await pg.program.account.artesania.fetch(piezaPda);

    console.log("📦 Datos extraídos de la PDA:");
    console.log("   - Artesano dueño:", piezaData.artesano.toBase58());
    console.log("   - ID de Pieza:", piezaData.idPieza.toString());
    console.log("   - Material:", piezaData.material);
    console.log("   - Región de origen:", piezaData.region);
    console.log("   - Estado actual:", piezaData.estado);
  } catch (e) {
    console.error("❌ Error al leer:", e.message);
  }

  // ==========================================
  // UPDATE: Cambiar el estado de la pieza
  // ==========================================
  try {
    console.log("\n--- 3. ACTUALIZANDO ESTADO ---");
    const txUpdate = await pg.program.methods
      .actualizarEstado(idPieza, "Vendida y Exportada")
      .accounts({
        pieza: piezaPda,
        artesano: pg.wallet.publicKey,
      })
      .rpc();
    console.log("✅ Transacción de actualización exitosa. Hash:", txUpdate);

    // Volvemos a leer para comprobar el cambio
    const piezaActualizada = await pg.program.account.artesania.fetch(piezaPda);
    console.log(
      "🔄 Nuevo estado verificado en la blockchain:",
      piezaActualizada.estado
    );
  } catch (e) {
    console.error("❌ Error al actualizar:", e.message);
  }

  // ==========================================
  // DELETE: Eliminar la cuenta y recuperar SOL
  // ==========================================
  try {
    console.log("\n--- 4. ELIMINANDO REGISTRO ---");
    const txDelete = await pg.program.methods
      .eliminarPieza(idPieza)
      .accounts({
        pieza: piezaPda,
        artesano: pg.wallet.publicKey,
      })
      .rpc();
    console.log(
      "✅ Transacción de eliminación (burn) exitosa. Hash:",
      txDelete
    );
    console.log(
      "💰 La cuenta fue cerrada y los tokens SOL de 'rent' han vuelto a tu wallet."
    );
  } catch (e) {
    console.error("❌ Error al eliminar:", e.message);
  }

  console.log("\n🚀 ¡Prueba del CRUD completada con éxito!");
})();
