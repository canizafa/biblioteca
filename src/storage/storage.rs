use std::{fs, path::Path};

use crate::{biblioteca::Biblioteca, errores::ErrorLibreria};


pub fn cargar_libreria(path: &str) -> Result<Biblioteca, ErrorLibreria> {

    if !Path::new(path).exists() {
      
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent).map_err(|_| ErrorLibreria::PathNoEncontrado)?;
        }
        
        let biblioteca_vacia = Biblioteca::new();
        if guardar_libreria(&biblioteca_vacia, path).is_err() {return Err(ErrorLibreria::PathNoEncontrado);}
        
        return Ok(biblioteca_vacia);
    }

  let content = std::fs::read_to_string(path).unwrap();
  let biblioteca: Biblioteca = serde_json::from_str(&content).unwrap();
  Ok(biblioteca)
}

pub fn guardar_libreria(libreria: &Biblioteca, path: &str) -> Result<(), ErrorLibreria> {

  if let Some(parent) = Path::new(path).parent() {
    fs::create_dir(parent).map_err(|_| ErrorLibreria::PathNoEncontrado)?;
  }

  let json = serde_json::to_string_pretty(libreria).unwrap();
  std::fs::write(path, json).unwrap();
  Ok(())
}