use core::fmt;



#[derive(Debug)]
pub enum ErrorLibro {
  CopiasInsuficientes,
}

impl fmt::Display for ErrorLibro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorLibro::CopiasInsuficientes => write!(f, "Cantidad de copias insuficientes")
        }
    }
}