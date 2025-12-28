use core::fmt;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::errores::ErrorPrestamo;

#[derive(Serialize, Deserialize, PartialEq)]
pub enum EstadoPrestamo {
  Devuelto(DateTime<Local>),
  EnCurso
}

#[derive(Serialize, Deserialize)]
pub struct Prestamo {
  id: Uuid,
  isbn_libro: u128,
  prestatario: String,
  estado_prestamo: EstadoPrestamo,
  fecha_prestamo: DateTime<Local>,
  fecha_devolucion: Option<DateTime<Local>>
}

impl Prestamo {
  pub fn new(id: Uuid, isbn_libro: u128, prestatario: String, fecha_prestamo: DateTime<Local>, estado_prestamo: EstadoPrestamo) -> Self {
    Self {
      id, 
      isbn_libro,
      prestatario,
      fecha_prestamo,
      estado_prestamo,
      fecha_devolucion: None
    }
  }
  // pub fn dias_transcurridos(&self) -> Result<(i64, EstadoPrestamo), ErrorPrestamo> {
  //   match self.estado_prestamo {
  //       EstadoPrestamo::Devuelto(date_time) => {
  //         Ok((
  //           date_time.signed_duration_since(self.fecha_prestamo).num_days(),
  //           EstadoPrestamo::Devuelto(date_time)
  //       ))
  //       },
  //       EstadoPrestamo::EnCurso => {
  //         let days = Local::now().signed_duration_since(self.fecha_prestamo).num_days();
  //         if days.is_negative() {
  //           Err(ErrorPrestamo::DiasNegativos)
  //         } else {
  //           Ok((
  //             days,
  //             EstadoPrestamo::EnCurso
  //           ))
  //         }
  //       }
  //   }
  // }
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
  pub fn agregar_fecha_devolucion(&mut self, fecha_devolucion: DateTime<Local>) -> Result<(), ErrorPrestamo> {
    if self.fecha_devolucion.is_some() {return Err(ErrorPrestamo::PrestamoDevuelto);}
    self.fecha_devolucion = Some(fecha_devolucion);
    Ok(())
  }
}

impl fmt::Display for Prestamo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
          "ISBN: {}. \nPrestatario: {}. \nFecha del préstamo: {}.",
          self.isbn_libro, self.prestatario, self.fecha_prestamo
        )
    }
}