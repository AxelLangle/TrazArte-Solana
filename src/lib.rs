use anchor_lang::prelude::*;

// SolPG generará automáticamente un ID cuando construyas (build) el proyecto.
declare_id!("9abLxRzjRsUNhdm5K2GvEkwSsX2je8FtS9MDderiJZNQ");

#[program]
pub mod trazarte {
    use super::*;

    // 1. CREATE (Inicializar la PDA de la artesanía)
    pub fn registrar_pieza(
        ctx: Context<RegistrarPieza>,
        id_pieza: u64,
        material: String,
        region: String,
    ) -> Result<()> {
        let pieza = &mut ctx.accounts.pieza;
        pieza.artesano = *ctx.accounts.artesano.key;
        pieza.id_pieza = id_pieza;
        pieza.material = material;
        pieza.region = region;
        pieza.estado = String::from("En taller"); // Estado por defecto

        msg!("Artesanía registrada exitosamente con el ID: {}", id_pieza);
        Ok(())
    }

    // 2. UPDATE (Actualizar el estado de la artesanía)
    pub fn actualizar_estado(
        ctx: Context<ActualizarPieza>,
        id_pieza: u64, // Lo pedimos porque se usa para derivar la PDA
        nuevo_estado: String,
    ) -> Result<()> {
        let pieza = &mut ctx.accounts.pieza;
        pieza.estado = nuevo_estado;

        msg!(
            "El estado de la pieza {} ha sido actualizado a: {}",
            id_pieza,
            pieza.estado
        );
        Ok(())
    }

    // 3. DELETE (Cerrar la cuenta y recuperar el rent)
    pub fn eliminar_pieza(
        _ctx: Context<EliminarPieza>, 
        id_pieza: u64 // Lo pedimos para derivar la PDA correcta a borrar
    ) -> Result<()> {
        msg!("El registro de la pieza {} ha sido eliminado (quemado) de la blockchain.", id_pieza);
        // No necesitamos escribir lógica aquí. 
        // La macro `close = artesano` en el contexto hace todo el trabajo.
        Ok(())
    }
}

// ==========================================
// ESTRUCTURAS DE CONTEXTO (Las validaciones)
// ==========================================

#[derive(Accounts)]
#[instruction(id_pieza: u64)]
pub struct RegistrarPieza<'info> {
    #[account(
        init,
        payer = artesano,
        space = 8 + 32 + 8 + 54 + 54 + 24, 
        seeds = [b"artesania", artesano.key().as_ref(), id_pieza.to_le_bytes().as_ref()],
        bump
    )]
    pub pieza: Account<'info, Artesania>,

    #[account(mut)]
    pub artesano: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(id_pieza: u64)]
pub struct ActualizarPieza<'info> {
    #[account(
        mut, 
        seeds = [b"artesania", artesano.key().as_ref(), id_pieza.to_le_bytes().as_ref()],
        bump,
        has_one = artesano
    )]
    pub pieza: Account<'info, Artesania>,
    
    #[account(mut)]
    pub artesano: Signer<'info>, 
}

#[derive(Accounts)]
#[instruction(id_pieza: u64)]
pub struct EliminarPieza<'info> {
    #[account(
        mut, 
        close = artesano, // ¡Esta es la instrucción mágica para borrar y devolver el SOL!
        seeds = [b"artesania", artesano.key().as_ref(), id_pieza.to_le_bytes().as_ref()],
        bump,
        has_one = artesano // Seguridad: solo el dueño puede borrarla
    )]
    pub pieza: Account<'info, Artesania>,
    
    #[account(mut)]
    pub artesano: Signer<'info>, 
}

// ==========================================
// EL ESTADO (Los datos que guardamos)
// ==========================================

#[account]
pub struct Artesania {
    pub artesano: Pubkey, // 32 bytes
    pub id_pieza: u64,    // 8 bytes
    pub material: String, // 4 bytes de prefijo + 50 bytes
    pub region: String,   // 4 bytes de prefijo + 50 bytes
    pub estado: String,   // 4 bytes de prefijo + 20 bytes
}
