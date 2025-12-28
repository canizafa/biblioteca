use crate::{biblioteca::Biblioteca, errores::ErrorRegistrarDevolucion};

pub fn registrar_devolucion(isbn: u128, prestatario: String, libreria: &mut Biblioteca) -> Result<(), ErrorRegistrarDevolucion> {
  if isbn <= 0 {return Err(ErrorRegistrarDevolucion::IsbnNulo);}
  if prestatario.trim().is_empty() {return Err(ErrorRegistrarDevolucion::PrestatarioNulo)}

  libreria.registrar_devolucion(isbn, prestatario)?;

  Ok(())

}