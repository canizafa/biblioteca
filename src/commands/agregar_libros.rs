use chrono::{Local, NaiveDate};
use uuid::Uuid;

use crate::{biblioteca::Biblioteca, errores::ErrorAgregarLibro, models::libro::{GeneroLiterario, Libro}};


pub fn agregar_libro(
  biblioteca: &mut Biblioteca,
  titulo: String, 
  autor: String, 
  isbn: u128, 
  anio_publicacion: NaiveDate, 
  genero: GeneroLiterario, 
  copias_disponibles: u8
) -> Result<(), ErrorAgregarLibro> {
    
  if titulo.trim().is_empty() {return Err(ErrorAgregarLibro::AutorNulo);}
  if autor.trim().is_empty() {return Err(ErrorAgregarLibro::AutorNulo);}
  if isbn <= 0 {return Err(ErrorAgregarLibro::IsbnNulo);}
  if Local::now().date_naive().signed_duration_since(anio_publicacion).num_days() < 0 {return Err(ErrorAgregarLibro::FechaInvalida);}
  if copias_disponibles <= 0 {return Err(ErrorAgregarLibro::CantidadDeCopiasInvalida);}

  let libro = Libro::new(
    Uuid::new_v4(),
    titulo, 
    autor, 
    isbn, 
    anio_publicacion, 
    genero, 
    copias_disponibles
    );

    biblioteca.incorporar_libro(libro)?;

  Ok(())
}