use core::fmt;

use crate::errores::ErrorLibro;
use chrono::NaiveDate;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum GeneroLiterario {
  Romance,
  Thriller,
  Policial,
  Novela,
  Cuento,
  Poesia
}
impl fmt::Display for GeneroLiterario {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GeneroLiterario::Cuento => write!(f, "Cuento"),
            GeneroLiterario::Novela => write!(f, "Novela"),
            GeneroLiterario::Romance => write!(f, "Romance"),
            GeneroLiterario::Thriller => write!(f, "Thriller"),
            GeneroLiterario::Policial => write!(f, "Policial"),
            GeneroLiterario::Poesia => write!(f, "Poesía"),
        }
    }
}



#[derive(Serialize, Deserialize)]
pub struct Libro {
  id: Uuid,
  titulo: String,
  autor: String,
  isbn: u128,
  anio_publicacion: NaiveDate,
  genero: GeneroLiterario,
  copias_disponibles: u8
}

impl Libro {
  pub fn new(
    id: Uuid,
    titulo: String,
    autor: String,
    isbn: u128,
    anio_publicacion: NaiveDate,
    genero: GeneroLiterario,
    copias_disponibles: u8
  ) -> Self {
    Self {
      id,
      titulo,
      autor,
      isbn,
      anio_publicacion,
      genero,
      copias_disponibles
    }
  }
  pub fn comparar_autor(&self, autor: String) -> bool {
    self.autor == autor
  }
  pub fn obtener_por_isbn(&mut self, isbn: u128) -> Option<&mut Libro> {
    if self.isbn == isbn {
      Some(self)
    } else {
      None
    }
  }
  pub fn disminuir_copias(&mut self) -> Result<u8, ErrorLibro> {
    if self.copias_disponibles > 0 {
      self.copias_disponibles -= 1;
      Ok(self.copias_disponibles)
    } else {
      Err(ErrorLibro::CopiasInsuficientes)
    }
  }
  pub fn aumentar_copias(&mut self) {
    self.copias_disponibles += 1;
  }
}
impl fmt::Display for Libro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
          "Titulo: {}. \nAutor: {}. \nISBN: {}. \nAño de publicación: {}. \n Género literario: {}.",
          self.titulo, self.autor, self.isbn, self.anio_publicacion, self.genero
        )
    }
}