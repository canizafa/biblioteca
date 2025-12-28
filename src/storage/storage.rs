use std::{fs, path::Path};

use crate::{biblioteca::Biblioteca, errores::ErrorApp};


pub fn cargar_libreria(path: &str) -> Result<Biblioteca, ErrorApp> {

    if !Path::new(path).exists() {
      
        if let Some(parent) = Path::new(path).parent() {
            if fs::create_dir_all(parent).is_err() {return Err(ErrorApp::DirectorioSinCrear); }
        }
        
        let biblioteca_vacia = Biblioteca::new();
        if guardar_libreria(&biblioteca_vacia, path).is_err() {return Err(ErrorApp::DirectorioSinCrear);}
        
        return Ok(biblioteca_vacia);
    }

  let content = std::fs::read_to_string(path).unwrap();
  let biblioteca: Biblioteca = serde_json::from_str(&content).unwrap();
  Ok(biblioteca)
}

pub fn guardar_libreria(libreria: &Biblioteca, path: &str) -> Result<(), ErrorApp> {

  if let Some(parent) = Path::new(path).parent() {
    if fs::create_dir(parent).is_err() {return Err(ErrorApp::DirectorioSinCrear);}
  }

  let json = serde_json::to_string_pretty(libreria).unwrap();
  std::fs::write(path, json).unwrap();
  Ok(())
}