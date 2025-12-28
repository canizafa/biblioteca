use core::fmt;

#[derive(Debug)]
pub enum ErrorLibro {
  CopiasInsuficientes,
  LibroDuplicado,
  TituloNulo,
  IsbnNulo,
  AutorNulo,
  FechaInvalida,
}
impl fmt::Display for ErrorLibro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorLibro::CopiasInsuficientes => write!(f, "Cantidad de copias insuficientes"),
            ErrorLibro::LibroDuplicado => write!(f, "El libro ya se encuentra en el Sistema"),
            ErrorLibro::TituloNulo => write!(f, "El título del libro no puede ser nulo"),
            ErrorLibro::IsbnNulo => write!(f,"El ISBN del libro no puede ser nulo"),
            ErrorLibro::AutorNulo => write!(f, "El nombre del autor del libro no puede ser nulo"),
            ErrorLibro::FechaInvalida => write!(f, "La fecha no puede ser mayor a la fecha actual"),
        }
    }
}

#[derive(Debug)]
pub enum ErrorPrestamo {
  PrestamoInexistente,
  EstadoInvalido,
  PrestamoExistente,
  PrestamoDevuelto,
  IsbnNulo,
  PrestatarioNulo,
}
impl fmt::Display for ErrorPrestamo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorPrestamo::PrestamoInexistente => write!(f,"El préstamo no existe"),
            ErrorPrestamo::EstadoInvalido => write!(f,"El cambio de estado no se pudo realizar"),
            ErrorPrestamo::PrestamoExistente => write!(f, "El prestamo ya fue ingresado"),
            ErrorPrestamo::PrestamoDevuelto => write!(f, "El prestamo ya fue devuelto"),
            ErrorPrestamo::IsbnNulo => write!(f, "El ISBN debe ser válido"),
            ErrorPrestamo::PrestatarioNulo => write!(f, "El nombre del prestatario no puede ser nulo"),
        }
    }
}

#[derive(Debug)]
pub enum ErrorLibreria {
    PathNoEncontrado,
    ContenidoInexistente,
    ParseoFallido,
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
            ErrorLibreria::PathNoEncontrado => write!(f,"No se encuentra el directorio de la aplicación"),
            ErrorLibreria::ContenidoInexistente => write!(f,"No se encuentra el contenido de la aplicación"),
            ErrorLibreria::ParseoFallido => write!(f,"El parseo de datos no se pudo realizar"),
            ErrorLibreria::Libro(e) => write!(f,"{}", e),
            ErrorLibreria::Prestamo(e) => write!(f,"{}",e),
        }
    }
}
