use crate::biblioteca::{Biblioteca};


pub fn cargar_libreria(path: &str) -> Result<Biblioteca, () > {
  let content = std::fs::read_to_string(path).unwrap();
  let biblioteca: Biblioteca = serde_json::from_str(&content).unwrap();
  Ok(biblioteca)
}

pub fn guardar_libreria(libreria: &Biblioteca, path: &str) -> Result<(), ()> {
  let json = serde_json::to_string_pretty(libreria).unwrap();
  std::fs::write(path, json).unwrap();
  Ok(())
}