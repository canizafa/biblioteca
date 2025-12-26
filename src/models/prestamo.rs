use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Serialize, Deserialize)]
pub enum EstadoPrestamo {
  Devuelto(DateTime<Local>),
  EnCurso,
  SinDevolver
}

#[derive(Serialize, Deserialize)]
pub struct Prestamo {
  id: Uuid,
  id_libro: Uuid,
  prestatario: String,
  fecha_prestamo: DateTime<Local>,
  estado_prestamo: EstadoPrestamo,
}

impl Prestamo {
}