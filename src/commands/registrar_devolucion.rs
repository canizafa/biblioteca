use colored::Colorize;

use crate::{biblioteca::Biblioteca, errores::{ErrorLibreria, ErrorPrestamo}};

pub fn registrar_devolucion(isbn: u128, prestatario: String, libreria: &mut Biblioteca) -> Result<(), ErrorLibreria> {
  if isbn <= 0 {return Err(ErrorLibreria::Prestamo(ErrorPrestamo::IsbnNulo));}
  if prestatario.trim().is_empty() {return Err(ErrorLibreria::Prestamo(ErrorPrestamo::PrestatarioNulo))}

  libreria.registrar_devolucion(isbn, prestatario)?;

  println!("{}\n", "Se ha registrado la devolución correctamente".bright_green().bold());

  Ok(())
}