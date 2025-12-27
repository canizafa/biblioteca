use crate::{biblioteca::Biblioteca, errores::ErrorListarLibro};


pub fn listar_libros_por_autor(libreria: &Biblioteca, autor: String) -> Result<(), ErrorListarLibro> {

  if autor.trim().is_empty() {return Err(ErrorListarLibro::AutorNulo)}
  
  let lista = libreria.listar_libros_por_autor(autor);
  if let Some(lista) = lista {
    lista.iter().for_each(|l| println!("{}",l));
  } else { return Err(ErrorListarLibro::ListaVacia) }

  Ok(())
}