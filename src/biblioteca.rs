use chrono::{Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    errores::{ErrorLibreria, ErrorLibro, ErrorPrestamo},
    models::{
        libro::{GeneroLiterario, Libro},
        prestamo::{self, Prestamo},
    },
};

#[derive(Serialize, Deserialize)]
pub struct Biblioteca {
    libros: Vec<Libro>,
    prestamos: Vec<Prestamo>,
}
impl Biblioteca {
    pub fn new() -> Self {
        Self {
            libros: Vec::new(),
            prestamos: Vec::new(),
        }
    }

    pub fn listar_libros_por_autor(&self, autor: String) -> Option<Vec<&Libro>> {
        let lista: Vec<&Libro> = self
            .libros
            .iter()
            .filter(|l| l.comparar_autor(&autor))
            .collect();

        if lista.is_empty() {
            return None;
        }

        Some(lista)
    }

    pub fn listar_prestamos_vigentes(&self) -> Option<Vec<&Prestamo>> {
        if self.prestamos.len() <= 0 {
            return None;
        }
        Some(self.prestamos.iter().filter(|p| p.esta_vigente()).collect())
    }

    pub fn registrar_prestamo(
        &mut self,
        isbn: u128,
        prestatario: String,
    ) -> Result<u8, ErrorLibreria> {
        if self
            .prestamos
            .iter()
            .any(|p| p.obtener_isbn_libro() == isbn && *p.obtener_prestatario() == prestatario)
        {
            return Err(ErrorLibreria::Prestamo(ErrorPrestamo::PrestamoExistente));
        }

        if let Some(libro) = self.libros.iter_mut().find(|l| l.obtener_isbn() == isbn) {
            let restantes = libro.disminuir_copias()?;

            let prestamo = Prestamo::new(
                Uuid::new_v4(),
                libro.obtener_isbn(),
                prestatario,
                Utc::now().date_naive(),
                prestamo::EstadoPrestamo::EnCurso,
                libro.obtener_genero().clone(),
            );

            self.prestamos.push(prestamo);

            Ok(restantes)
        } else {
            todo!("hacer camino de else en registrar prestamo");
        }
    }

    pub fn registrar_devolucion(
        &mut self,
        isbn: u128,
        prestatario: String,
    ) -> Result<(), ErrorLibreria> {
        let prestamo_buscado = self
            .prestamos
            .iter_mut()
            .find(|p| p.obtener_isbn_libro() == isbn && *p.obtener_prestatario() == prestatario)
            .ok_or(ErrorLibreria::Prestamo(ErrorPrestamo::PrestamoInexistente))?;

        prestamo_buscado.cambiar_estado(prestamo::EstadoPrestamo::Devuelto(Utc::now().date_naive()))?;
        prestamo_buscado.agregar_fecha_devolucion(Utc::now().date_naive())?;

        if let Some(libro) = self.libros.iter_mut().find(|l| l.obtener_isbn() == isbn) {
            libro.aumentar_copias();
        }

        Ok(())
    }

    pub fn incorporar_libro(&mut self, libro: Libro) -> Result<(), ErrorLibreria> {
        if self
            .libros
            .iter()
            .any(|l| l.obtener_isbn() == libro.obtener_isbn())
        {
            return Err(ErrorLibreria::Libro(ErrorLibro::LibroDuplicado));
        } else {
            self.libros.push(libro);
            Ok(())
        }
    }

    pub fn devolver_generos_prestamos(&self) -> Option<Vec<&GeneroLiterario>> {
        if self.prestamos.is_empty() {
            return None;
        }

        let mut vec = Vec::new();

        self.prestamos
            .iter()
            .for_each(|p| vec.push(p.obtener_genero()));

        Some(vec)
    }
    pub fn calcular_promedio_dias_prestamo(&self) -> Option<f64> {
        if self.prestamos.len() == 0 {
            return None;
        }
        let obtener_dias: f64 = self
            .prestamos
            .iter()
            .filter_map(|p| p.dias_transcurridos())
            .sum();
        let cant_prestamos = self
            .prestamos
            .iter()
            .filter(|p| !p.esta_vigente())
            .count() as f64;

        if cant_prestamos == 0 as f64 {
            None
        } else {
            Some(obtener_dias/cant_prestamos)
        }

    }
}
