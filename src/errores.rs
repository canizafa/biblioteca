use core::fmt;

#[derive(Debug)]
pub enum ErrorLibro {
  CopiasInsuficientes,
  LibroInexistente,
}
impl fmt::Display for ErrorLibro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorLibro::CopiasInsuficientes => write!(f, "Cantidad de copias insuficientes"),
            ErrorLibro::LibroInexistente => write!(f, "No existe el libro buscado"),
        }
    }
}

#[derive(Debug)]
pub enum ErrorPrestamo {
  DiasNegativos,
  PrestamoInexistente,
  EstadoInvalido,
}
impl fmt::Display for ErrorPrestamo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorPrestamo::DiasNegativos => write!(f, "Cantidad de días transcurridos inválido"),
            ErrorPrestamo::PrestamoInexistente => write!(f,"El préstamo no existe"),
            ErrorPrestamo::EstadoInvalido => write!(f,"El cambio de estado no se pudo realizar"),
        }
    }
}

#[derive(Debug)]
pub enum ErrorLibreria {
    Libro(ErrorLibro),
    Prestamo(ErrorPrestamo),
}
impl From<ErrorLibro> for ErrorLibreria {
    fn from(e: ErrorLibro) -> Self {
        ErrorLibreria::Libro(e)
    }
}
impl From<ErrorPrestamo> for ErrorLibreria {
    fn from(e: ErrorPrestamo) -> Self {
        ErrorLibreria::Prestamo(e)
    }
}
impl fmt::Display for ErrorLibreria {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorLibreria::Libro(e) => write!(f,"Error de libro {}", e),
            ErrorLibreria::Prestamo(e) => write!(f,"Error de prestamo {}",e),
        }
    }
}