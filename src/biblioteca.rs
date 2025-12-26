use crate::models::{libro::Libro, prestamo::Prestamo};


pub enum BusquedaLibro<'a> {
  BusquedaISBN(u128),
  BusquedaAutorTitulo((&'a String, &'a String))
}


pub struct Biblioteca {
  libros: Vec<Libro>,
  prestamos: Vec<Prestamo>,
}


impl Biblioteca {
}