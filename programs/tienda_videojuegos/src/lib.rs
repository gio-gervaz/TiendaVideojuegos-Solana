use anchor_lang::prelude::*;

declare_id!("GameStore1111111111111111111111111111111111");

#[program]
pub mod tienda_videojuegos {
    use super::*;

    pub fn crear_tienda(ctx: Context<CrearTienda>, nombre: String) -> Result<()> {
        let tienda = &mut ctx.accounts.tienda;
        tienda.nombre = nombre;
        Ok(())
    }

    pub fn agregar_juego(ctx: Context<AgregarJuego>, nombre: String, precio: u64) -> Result<()> {
        let juego = &mut ctx.accounts.juego;
        juego.nombre = nombre;
        juego.precio = precio;
        Ok(())
    }
}

#[account]
pub struct Tienda {
    pub nombre: String,
}

#[account]
pub struct Juego {
    pub nombre: String,
    pub precio: u64,
}

#[derive(Accounts)]
pub struct CrearTienda<'info> {
    #[account(init, payer = user, space = 8 + 64)]
    pub tienda: Account<'info, Tienda>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AgregarJuego<'info> {
    #[account(init, payer = user, space = 8 + 64)]
    pub juego: Account<'info, Juego>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}
