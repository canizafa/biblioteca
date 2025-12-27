use core::fmt;

#[derive(Debug)]
pub enum ErrorLibro {
  CopiasInsuficientes,
  LibroInexistente,
  LibroDuplicado
}
impl fmt::Display for ErrorLibro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorLibro::CopiasInsuficientes => write!(f, "Cantidad de copias insuficientes"),
            ErrorLibro::LibroInexistente => write!(f, "No existe el libro buscado"),
            ErrorLibro::LibroDuplicado => write!(f, "El libro ya se encuentra en el Sistema"),
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


#[derive(Debug)]
pub enum ErrorAgregarLibro {
    IsbnNulo,
    AutorNulo,
    FechaInvalida,
    CantidadDeCopiasInvalida,
    TituloNulo,
    Libreria(ErrorLibreria)
}
impl fmt::Display for ErrorAgregarLibro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorAgregarLibro::IsbnNulo => write!(f, "El ISBN no puede ser 0"),
            ErrorAgregarLibro::AutorNulo => write!(f, "EL autor no puede estar vacío"),
            ErrorAgregarLibro::FechaInvalida => write!(f, "La fecha no puede ser superior a la fecha actual"),
            ErrorAgregarLibro::CantidadDeCopiasInvalida => write!(f, "La cantidad de copias no puede ser menor o igual a 0"),
            ErrorAgregarLibro::TituloNulo => write!(f, "El título no puede estar vacío"),
            ErrorAgregarLibro::Libreria(e) => write!(f,"Error de libreria: {}",e),
        }
    }
}
impl From<ErrorLibreria> for ErrorAgregarLibro {
    fn from(e: ErrorLibreria) -> Self {
        ErrorAgregarLibro::Libreria(e)
    }
}

#[derive(Debug)] 
pub enum ErrorListarLibro {
    AutorNulo,
    ListaVacia,
}
impl fmt::Display for ErrorListarLibro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorListarLibro::AutorNulo => write!(f,"El autor a buscar no puede ser nulo"),
            ErrorListarLibro::ListaVacia => write!(f, "La lista por autor se encuentra vacía"),
        }
    }
}

pub enum ErrorPrestamosVigentes {
    ListaVacia,   
}
impl fmt::Display for ErrorPrestamosVigentes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorPrestamosVigentes::ListaVacia => write!(f,"La lista de prestamos vigentes se encuetra vacía"),
        }
    }
}

pub enum ErrorRegistrarPrestamo {
    IsbnNulo,
    PrestatarioNulo,
    Libreria(ErrorLibreria),
}
impl fmt::Display for ErrorRegistrarPrestamo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorRegistrarPrestamo::IsbnNulo => write!(f,"El isbn no puede ser nulo"),
            ErrorRegistrarPrestamo::PrestatarioNulo => write!(f, "El prestatario no puede ser nulo"),
            ErrorRegistrarPrestamo::Libreria(e) => write!(f, "Error de prestamo: {}", e),
        }
    }
}

impl From<ErrorLibreria> for ErrorRegistrarPrestamo {
    fn from(e: ErrorLibreria) -> Self {
        ErrorRegistrarPrestamo::Libreria(e)
    }
}

pub enum ErrorRegistrarDevolucion {
    PrestamoNoEncontrado,
    IsbnNulo,
    PrestatarioNulo,
    Libreria(ErrorLibreria),
}

impl fmt::Display for ErrorRegistrarDevolucion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorRegistrarDevolucion::PrestamoNoEncontrado => write!(f,"No se ha encontrado el prestamo"),
            ErrorRegistrarDevolucion::IsbnNulo => write!(f, "El ISBN no puede ser nulo"),
            ErrorRegistrarDevolucion::PrestatarioNulo => write!(f, "El prestatario no puede ser nulo"),
            ErrorRegistrarDevolucion::Libreria(e) => write!(f, "Error de libreria: {}", e),
        }
    }
}

impl From<ErrorLibreria> for ErrorRegistrarDevolucion {
    fn from(e: ErrorLibreria) -> Self {
        ErrorRegistrarDevolucion::Libreria(e)
    }
}


#[derive(Debug)]
pub enum ErrorComando {
    AgregarLibro(ErrorAgregarLibro),
    ListarLibro(ErrorListarLibro),
}