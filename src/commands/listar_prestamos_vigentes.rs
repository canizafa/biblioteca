use colored::Colorize;

use crate::biblioteca::Biblioteca;

pub fn listar_prestamos_vigentes(libreria: &Biblioteca) {
    if let Some(lista_prestamos) = libreria.listar_prestamos_vigentes() {
        lista_prestamos.iter().for_each(|p| println!("{}\n", p));
    } else {
        println!(
            "{}\n",
            "La lista de prestamos vigentes se encuentra vacia"
                .blue()
                .bold()
        );
    }
}
