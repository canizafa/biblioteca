use crate::{biblioteca::Biblioteca, errores::ErrorPrestamosVigentes};


pub fn listar_prestamos_vigentes(libreria: &Biblioteca) -> Result<(), ErrorPrestamosVigentes> {

  let lista_prestamos = libreria.listar_prestamos_vigentes();
  if lista_prestamos.len() <= 0 {return Err(ErrorPrestamosVigentes::ListaVacia);}

  lista_prestamos.iter().for_each(|p| println!("{}", p));
  Ok(())
}