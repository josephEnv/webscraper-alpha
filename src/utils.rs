use clap::Parser;
use std::fs::{self};
use std::path::Path;
use thirtyfour::prelude::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Tu ruc es el usuario
    #[arg(short, long)]
    pub user: String,

    /// Aqui va tu contraseña
    #[arg(short, long)]
    pub password: String,

    /// Año de la consulta
    #[arg(short, long)]
    pub year: String,

    /// Mes de la consulta
    #[arg(short, long)]
    pub month: String,

    /// Dia de la consulta
    #[arg(short, long)]
    pub day: String,

    /// Directorio de descargas
    #[arg(short, long = "download-dir")]
    pub init_dir: String,
}

pub fn sanitize_name(file_name: &str) -> String {
    // Convierte las mayúsculas a minúsculas
    let lower_case = file_name.to_lowercase();

    // Reemplaza los espacios con guiones bajos
    let no_spaces = lower_case.replace(" ", "_");

    // Quita las tildes
    let sanitized_name = no_spaces
        .chars()
        .map(|c| match c {
            'á' => 'a',
            'é' => 'e',
            'í' => 'i',
            'ó' => 'o',
            'ú' => 'u',
            'Á' => 'A',
            'É' => 'E',
            'Í' => 'I',
            'Ó' => 'O',
            'Ú' => 'U',
            _ => c,
        })
        .collect::<String>();

    // Encuentra la posición del último '.' en el nombre del archivo
    if let Some(dot_index) = sanitized_name.rfind('.') {
        // Devuelve el nombre del archivo sin la extensión
        sanitized_name[..dot_index].to_owned()
    } else {
        // Si no hay extensión, devuelve el nombre sin cambios
        sanitized_name
    }
}

pub fn create_directory(args: &Args, path: &str) {
    let full_path = Path::new(&args.init_dir.clone())
        .join(args.year.clone())
        .join(args.user.clone())
        .join(args.month.clone())
        .join(path);

    if let Err(e) = fs::create_dir_all(&full_path) {
        eprintln!("Error al crear el directorio: {}", e);
    }
}

pub async fn select(elms: &[WebElement], texto: String) -> WebDriverResult<Option<WebElement>> {
    for elemento in elms {
        let texto_elm = elemento.text().await?;
        if texto_elm == texto {
            return Ok(Some(elemento.clone()));
        }
    }

    Ok(None)
}

