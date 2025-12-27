use chrono::Local;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{errores::{ErrorLibreria, ErrorLibro, ErrorPrestamo}, models::{libro::Libro, prestamo::{self, Prestamo}}};


#[derive(Serialize, Deserialize)]
pub struct Biblioteca {
  libros: Vec<Libro>,
  prestamos: Vec<Prestamo>,
}


impl Biblioteca {
  pub fn new() -> Self {
    Self {
      libros: Vec::new(),
      prestamos: Vec::new()
    }
  }
  pub fn listar_libros_por_autor(&self, autor: String) -> Option<Vec<&Libro>> {
    let lista: Vec<&Libro> = self.libros.iter().filter(|l| l.comparar_autor(&autor)).collect();
    if lista.is_empty() {return  None;}
    Some(lista)
  }
  pub fn listar_prestamos_vigentes(&self) -> Vec<&Prestamo> {
    self.prestamos.iter().filter(|p| p.esta_vigente()).collect()
  }
  pub fn registrar_prestamo(&mut self, isbn: u128, prestatario: String) -> Result<u8, ErrorLibreria> {
    
    let libro_buscado = self.libros
        .iter_mut()
        .find_map(|l| l.obtener_por_isbn(isbn)).ok_or(ErrorLibro::LibroInexistente)?;

    if self.prestamos
      .iter()
      .any(|p| p.obtener_isbn_libro() == isbn && *p.obtener_prestatario() == prestatario) {
        return Err(ErrorLibreria::Prestamo(ErrorPrestamo::PrestamoExistente));
      }

    let restantes = libro_buscado.disminuir_copias()?;

    let prestamo = Prestamo::new(Uuid::new_v4(),
     libro_buscado.obtener_isbn(),
     prestatario, 
     Local::now(), 
     prestamo::EstadoPrestamo::EnCurso
    );

    self.prestamos.push(prestamo);

    Ok(restantes)
  }
  pub fn registrar_devolucion(&mut self, isbn: u128, prestatario: String) -> Result<(), ErrorLibreria> {
    
    let prestamo_buscado = self.prestamos
      .iter_mut()
      .find(|p| p.obtener_isbn_libro() == isbn && *p.obtener_prestatario() == prestatario)
      .ok_or(ErrorLibreria::Prestamo(ErrorPrestamo::PrestamoInexistente))?;

    prestamo_buscado.cambiar_estado(prestamo::EstadoPrestamo::Devuelto(Local::now()))?;
    prestamo_buscado.agregar_fecha_devolucion(Local::now())?;

    let libro_devuelto = self.libros.iter_mut()
        .find_map(|l| l.obtener_por_isbn(isbn))
        .ok_or(ErrorLibreria::Libro(ErrorLibro::LibroInexistente))?;

    libro_devuelto.aumentar_copias();

    Ok(())
  }
  pub fn incorporar_libro(&mut self, libro: Libro) -> Result<(), ErrorLibreria> {
    
    if self.libros
      .iter()
      .any(|l| l.obtener_isbn() == libro.obtener_isbn()) {return Err(ErrorLibreria::Libro(ErrorLibro::LibroDuplicado));}
    else {
      self.libros.push(libro);
      Ok(())
    }
  }
}

