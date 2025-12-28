mod models;
mod storage;
mod biblioteca;
mod errores;
mod commands;

use std::env;
use clap::{Parser, Subcommand};
use colored::Colorize;
use crate::biblioteca::Biblioteca;
use crate::commands::{
    agregar_libros::agregar_libro,
    listar_libros::listar_libros_por_autor,
    listar_prestamos_vigentes::listar_prestamos_vigentes,
    registrar_devolucion::registrar_devolucion,
    registrar_prestamos::registrar_prestamo,
    reporte_genero::reporte_genero,
};
use crate::errores::ErrorLibreria;
use crate::models::libro::GeneroLiterario;
use crate::storage::storage::{cargar_libreria, guardar_libreria};


#[derive(Parser)]
#[command(name = "Mi Librero")]
#[command(version = "1.0")]
#[command(about = "Maneja tus prestamos de libros", long_about = None)]
struct Cli {
    #[command(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {

    #[command(name="agregar-libro", about="Agrega un libro al catálogo")]
    AgregarLibro {
        #[arg(long)]
        isbn: u128,
        #[arg(long)]
        anio: u128,
        #[arg(long)]
        genero: String,
        #[arg(long)]
        autor: String,
        #[arg(long)]
        titulo: String,
        #[arg(long)]
        disponibles: u8
    },

    #[command(name="listar-libros", about="Lista los libros del catalógo")]
    ListarLibros {
        #[arg(long)]
        autor: String,
    },

    #[command(name="listar-prestamos", about="Lista los prestamos vigentes")]
    ListarPrestamos,

    #[command(name="registrar-prestamo", about="Registra un prestamo de un libro")]
    RegistrarPrestamo {
        #[arg(long)]
        isbn: u128,
        #[arg(long)]
        prestatario: String
    },

    #[command(name="registrar-devolucion", about="Registra la devolución de un préstamo")]
    RegistrarDevolucion {
        #[arg(long)]
        isbn: u128,
        #[arg(long)]
        prestatario: String
    },

    #[command(name="reporte-generos", about="Consulte el reporte del género más solicitado")]
    ReporteGenero,
}

fn main() -> Result<(), ErrorLibreria> {
    
    println!("\n{}\n", "**********".bright_blue().bold());

    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());

    let path = format!("{}/librero/libreria.json", home);

    let mut libreria = cargar_libreria(&path).unwrap_or_else(|_| Biblioteca::new());
    let cli = Cli::parse();

    match cli.action {
        Action::AgregarLibro { 
            isbn, 
            anio, 
            genero, 
            autor, 
            titulo, 
            disponibles 
        } => {
            let genero: GeneroLiterario = match genero.as_str() {
                "romance" => GeneroLiterario::Romance,
                "thriller" => GeneroLiterario::Thriller,
                "policial" => GeneroLiterario::Policial,
                "novela" => GeneroLiterario::Novela,
                "cuento" => GeneroLiterario::Cuento,
                "poesia" => GeneroLiterario::Poesia,
                _ => {GeneroLiterario::Cuento}
            };

            agregar_libro(
                &mut libreria, 
                titulo, 
                autor, 
                isbn, 
                anio, 
                genero, 
                disponibles).unwrap_or_else(|e| println!("{}",e));
        },
        Action::ListarLibros { autor } => {
            listar_libros_por_autor(&libreria, autor)
                .unwrap_or_else(|e| println!("{}",e))
        },
        Action::ListarPrestamos => {
            listar_prestamos_vigentes(&libreria)
            },
        Action::RegistrarPrestamo { isbn, prestatario } => {
            registrar_prestamo(isbn, prestatario, &mut libreria)
                .unwrap_or_else(|e| println!("{}",e));
        },
        Action::RegistrarDevolucion { isbn, prestatario } => {
            registrar_devolucion(isbn, prestatario, &mut libreria)
                .unwrap_or_else(|e| println!("{}",e));
        },
        Action::ReporteGenero => reporte_genero(&libreria),
    }

    guardar_libreria(&libreria, &path).unwrap_or_else(|e| println!("{}", e));
    
    println!("{}\n", "**********".bright_blue().bold());

    Ok(())
}
