use crate::{biblioteca::Biblioteca, errores::ErrorRegistrarPrestamo};


pub fn registrar_prestamo(isbn: u128, prestatario: String, libreria: &mut Biblioteca) -> Result<(), ErrorRegistrarPrestamo> {

  if isbn <= 0 {return Err(ErrorRegistrarPrestamo::IsbnNulo);}
  if prestatario.trim().is_empty() {return Err(ErrorRegistrarPrestamo::PrestatarioNulo);}

  libreria.registrar_prestamo(isbn, prestatario)?;


  Ok(())
}