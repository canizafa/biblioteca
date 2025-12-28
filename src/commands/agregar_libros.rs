use chrono::{Datelike, Local};
use colored::Colorize;
use uuid::Uuid;

use crate::{biblioteca::Biblioteca, errores::{ErrorLibreria, ErrorLibro}, models::libro::{GeneroLiterario, Libro}};


pub fn agregar_libro(
  biblioteca: &mut Biblioteca,
  titulo: String, 
  autor: String, 
  isbn: u128, 
  anio_publicacion: u128,
  genero: GeneroLiterario, 
  copias_disponibles: u8
) -> Result<(), ErrorLibreria> {
    
  if titulo.trim().is_empty() {return Err(ErrorLibreria::Libro(ErrorLibro::AutorNulo));}
  if autor.trim().is_empty() {return Err(ErrorLibreria::Libro(ErrorLibro::TituloNulo));}
  if isbn <= 0 {return Err(ErrorLibreria::Libro(ErrorLibro::IsbnNulo));}
  if Local::now().year() - anio_publicacion as i32 <= 0 {return Err(ErrorLibreria::Libro(ErrorLibro::FechaInvalida));}
  if copias_disponibles <= 0 {return Err(ErrorLibreria::Libro(ErrorLibro::CopiasInsuficientes));}

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
    println!("{}", "Se ha agregado correctamente el libro dentro del catalogo".green());

  Ok(())
}