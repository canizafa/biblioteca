use core::fmt;

use crate::{errores::ErrorPrestamo, models::libro::GeneroLiterario};
use chrono::{NaiveDate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, PartialEq)]
pub enum EstadoPrestamo {
    Devuelto(NaiveDate),
    EnCurso,
}

#[derive(Serialize, Deserialize)]
pub struct Prestamo {
    id: Uuid,
    isbn_libro: u128,
    prestatario: String,
    estado_prestamo: EstadoPrestamo,
    fecha_inicio: NaiveDate,
    fecha_devolucion: Option<NaiveDate>,
    genero_literario: GeneroLiterario,
}

impl Prestamo {
    pub fn new(
        id: Uuid,
        isbn_libro: u128,
        prestatario: String,
        fecha_inicio: NaiveDate,
        estado_prestamo: EstadoPrestamo,
        genero_literario: GeneroLiterario,
    ) -> Self {
        Self {
            id,
            isbn_libro,
            prestatario,
            fecha_inicio,
            estado_prestamo,
            fecha_devolucion: None,
            genero_literario,
        }
    }
    pub fn dias_transcurridos(&self) -> Option<f64> {
        if let Some(fecha_devolucion) = self.fecha_devolucion {
            let total = (fecha_devolucion - self.fecha_inicio).num_days() as f64;
            Some(total)
        } else {
            None
        }
    }

    pub fn obtener_genero(&self) -> &GeneroLiterario {
        &self.genero_literario
    }

    pub fn esta_vigente(&self) -> bool {
        match self.estado_prestamo {
            EstadoPrestamo::Devuelto(_) => false,
            EstadoPrestamo::EnCurso => true,
        }
    }

    pub fn obtener_prestatario(&self) -> &String {
        &self.prestatario
    }

    pub fn obtener_isbn_libro(&self) -> u128 {
        self.isbn_libro
    }

    pub fn cambiar_estado(&mut self, estado_nuevo: EstadoPrestamo) -> Result<(), ErrorPrestamo> {
        if self.estado_prestamo == estado_nuevo {
            Err(ErrorPrestamo::EstadoInvalido)
        } else {
            self.estado_prestamo = estado_nuevo;
            Ok(())
        }
    }

    pub fn agregar_fecha_devolucion(
        &mut self,
        fecha_devolucion: NaiveDate,
    ) -> Result<(), ErrorPrestamo> {
        if self.fecha_devolucion.is_some() {
            return Err(ErrorPrestamo::PrestamoDevuelto);
        }
        self.fecha_devolucion = Some(fecha_devolucion);
        Ok(())
    }
}

impl fmt::Display for Prestamo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ISBN: {}. \nPrestatario: {}. \nFecha del préstamo: {}.",
            self.isbn_libro, self.prestatario, self.fecha_inicio
        )
    }
}
