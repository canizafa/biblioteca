use colored::Colorize;

use crate::{biblioteca::Biblioteca, errores::{ErrorLibreria, ErrorLibro}};


pub fn listar_libros_por_autor(libreria: &Biblioteca, autor: String) -> Result<(), ErrorLibreria> {

  if autor.trim().is_empty() {return Err(ErrorLibreria::Libro(ErrorLibro::AutorNulo))}
  
  let lista = libreria.listar_libros_por_autor(autor);
  if let Some(lista) = lista {
    lista.iter().for_each(|l| println!("{}\n{}",l, "---------------".purple().bold()));

  } else { println!("{}", "La lista de libros se encuentra vacía".blue().bold()) }

  Ok(())
}