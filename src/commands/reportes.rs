use std::collections::HashMap;

use colored::Colorize;

use crate::{biblioteca::Biblioteca, models::libro::GeneroLiterario};

fn reporte_genero(libreria: &Biblioteca) {
    let mut hashmap = HashMap::from([
        (GeneroLiterario::Cuento, 0),
        (GeneroLiterario::Novela, 0),
        (GeneroLiterario::Poesia, 0),
        (GeneroLiterario::Policial, 0),
        (GeneroLiterario::Romance, 0),
        (GeneroLiterario::Thriller, 0),
    ]);

    if let Some(generos) = libreria.devolver_generos_prestamos() {
        generos.iter().for_each(|g| {
            if let Some(valor) = hashmap.get_mut(g) {
                *valor += 1;
            }
        });
    } else {
        println!(
            "{}",
            "No se pudo realizar el listado de géneros".red().bold()
        );
    }

    let mut genero_maximo: Option<GeneroLiterario> = None;
    let mut max: i32 = -1;

    for (clave, valor) in hashmap {
        if valor > max {
            max = valor;
            genero_maximo = Some(clave);
        }
    }

    if let Some(genero) = genero_maximo {
        println!(
            "{}{}{}{}{}",
            "El género que más fue solicitado fue: "
                .bold()
                .bright_blue(),
            genero,
            " con una cantidad de: ".bold().blue(),
            max,
            " préstamos\n".bold().bright_blue()
        );
    } else {
        println!("{}", "No se pudo realizar el reporte".red().bold());
    }
}

fn reporte_dias_prestamo_promedio(libreria: &Biblioteca) {
    if let Some(promedio) = libreria.calcular_promedio_dias_prestamo() {
        println!(
            "{}{}",
            "El promedio de días prestado es: ".bold().bright_blue(),
            promedio
        );
    } else {
        println!(
            "{}",
            "La lista de préstamos se encuentra vacía"
                .bold()
                .bright_blue()
        );
    }
}

pub fn reporte_general(libreria: &Biblioteca) {
    reporte_dias_prestamo_promedio(libreria);
    reporte_genero(libreria);
}
