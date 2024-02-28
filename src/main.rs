mod utils;

use async_openai::config::OpenAIConfig;
use async_openai::{types::CreateTranscriptionRequestArgs, Client};
use clap::Parser;
use serde_json::json;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;
use thirtyfour::prelude::*;
use utils::{Args, create_directory, sanitize_name, select};

#[warn(unused_mut)]
#[warn(unused_variables)]
#[warn(unused_must_use)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    //Creo el servidor
    let args = Args::parse();


    let chrome = DesiredCapabilities::chrome();
    let mut caps = chrome;

    let d_dir = Path::new(&args.init_dir);

    create_directory(&args, "factura");
    create_directory(&args, "notas_de_credito");
    create_directory(&args, "notas_de_debito");
    create_directory(&args, "comprobante_de_retencion");

    let prefs = json!({
        "download.default_directory": "C:\\Datos"
    });
    caps.add_arg("--headless").unwrap();
    caps.add_experimental_option("prefs", prefs).unwrap();
    let driver = WebDriver::new("http://localhost:9515/wd/hub", caps).await?;

    println!("{}", "Llendo a la pagina principal...");
    // Me dirijo el login
    driver
        .goto("https://srienlinea.sri.gob.ec/sri-en-linea/contribuyente/perfil")
        .await?;

    thread::sleep(Duration::from_secs(1));

    let caja_login = driver.find(By::Id("usuario")).await?;
    let caja_clave = driver.find(By::Id("password")).await?;
    let boton_ingreso = driver.find(By::Id("kc-login")).await?;

    println!("Llenando los datos del login");
    //Me logeo
    caja_login.send_keys(args.user.clone()).await?;
    caja_clave.send_keys(args.password.clone()).await?;
    boton_ingreso.click().await?;

    thread::sleep(Duration::from_secs(1));

    println!("Redirigiendo");
    //Me dirijo a las descargas
    driver.goto("https://srienlinea.sri.gob.ec/tuportal-internet/accederAplicacion.jspa?redireccion=57&idGrupo=55").await?;

    thread::sleep(Duration::from_secs(1));

    loop {
        loop {
            let _navbar = driver.find(By::Id("pnlBody")).await?;
            driver
                .execute(
                    r#"
                document.getElementById("pnlBody").remove()
            "#,
                    Vec::new(),
                )
                .await?;
            let anio = driver.find(By::Css("select.sri-input-combo-anio")).await?;
            let mes = driver.find(By::Css("select.sri-input-combo-mes")).await?;
            let dia = driver.find(By::Css("select.sri-input-combo-dia")).await?;

            let anio_text = anio.find_all(By::Tag("option")).await?;
            let mes_text = mes.find_all(By::Tag("option")).await?;
            let dia_text = dia.find_all(By::Tag("option")).await?;

            let select_anio = select(&anio_text, args.year.clone()).await?;
            let select_mes = select(&mes_text, args.month.clone()).await?;
            let select_dia = select(&dia_text, args.day.clone()).await?;

            println!("Seleccionando fechas");
            //Selecciono la fecha
            match (select_anio, select_mes, select_dia) {
                (Some(elem), Some(elem_mes), Some(elem_dia)) => {
                    elem.click().await?;
                    elem_mes.click().await?;
                    elem_dia.click().await?;
                }
                _ => {
                    println!("No hay elementos coincidentes")
                }
            }

            println!("Enviado formulario...");
            //Resuelvo el captcha
            let button_captcha = driver.find(By::Id("btnRecaptcha")).await?;
            button_captcha.click().await?;
            driver.enter_frame(2).await?;

            thread::sleep(Duration::from_secs(1));

            println!("Abriendo captcha");

            let audio_button = driver.find(By::Id("recaptcha-audio-button")).await;

            match audio_button {
                Ok(elm) => {
                    elm.click().await?;
                    break; //Mtmma*0820
                }
                Err(_) => {
                    println!("Intentandolo de nuevo");
                    driver.refresh().await?;
                }
            }
        }

        driver.enter_default_frame().await?;

        thread::sleep(Duration::from_secs(1));

        driver.enter_frame(2).await?;

        thread::sleep(Duration::from_secs(1));

        let button_container = driver
            .find(By::ClassName("rc-audiochallenge-tdownload"))
            .await;

        match button_container {
            Ok(elm) => {
                let button = elm.find(By::Tag("a")).await?;
                let attr = button.attr("href").await?;
                let link = attr.unwrap();

                println!("link: {}", link);

                let response = reqwest::get(link).await?;

                if response.status().is_success() {
                    let mut file = File::create("audio.mp3")?;
                    let content = response.bytes().await?;

                    // Escribe el contenido en el archivo
                    std::io::copy(&mut content.as_ref(), &mut file)?;

                    println!("¡Archivo descargado exitosamente!");
                } else {
                    println!(
                        "Error: La solicitud no fue exitosa. Código de estado: {}",
                        response.status()
                    );
                }

                break;
            }
            Err(_) => {
                println!("Error, intentandolo de nuevo");
                driver.refresh().await?;
            }
        }
    }

    thread::sleep(Duration::from_secs(2));

    let api_key = "<OPENAI_API_KEY>";
    let config = OpenAIConfig::new().with_api_key(api_key);
    let client = Client::with_config(config);

    let request = CreateTranscriptionRequestArgs::default()
        .file("audio.mp3")
        .model("whisper-1")
        .language("es")
        .build();

    let response = client.audio().transcribe(request.unwrap()).await;

    println!("Pasando el audio a texto...");
    let text_response = response.unwrap().text;

    println!("{}", text_response);
    println!("{}", text_response.len());

    let input_response = driver.find(By::Id("audio-response")).await?;



    input_response.send_keys(text_response).await?;

    let verify_button = driver.find(By::Id("recaptcha-verify-button")).await?;
    verify_button.click().await?;

    println!("nice");

    driver.enter_default_frame().await?;

    thread::sleep(Duration::from_secs(2));

    let table = driver.find(By::Id("frmPrincipal:tablaCompRecibidos_data")).await;
    
    match table {
        Ok(table) => {

            loop {
                

                let trs = match table.find_all(By::Tag("tr")).await {
                    Ok(trs) => trs,
                    Err(_) => {
                        println!("Error al encontrar las filas de la tabla.");
                        break;
                    }
                };

                println!("{}", trs.len());

                for element in trs {
                    let tds = match element.find_all(By::Tag("td")).await {
                        Ok(tds) => tds,
                        Err(_) => continue,
                    };

                    if let Some(xml) = tds.get(7) {
                        if xml.wait_until().enabled().await.is_ok() && xml.is_clickable().await.is_ok() {
                            let pdf = tds.get(8).unwrap();
                            loop {

                                xml.click().await?;
                                pdf.click().await?;
                                
                                let mut true_file = false;
                                thread::sleep(Duration::from_millis(500));
    
                                if let Ok(clave_acceso_cont) = tds[3].clone().find(By::Tag("a")).await {
                                    if let Ok(clave_acceso) = clave_acceso_cont.text().await {
                                        println!("{}", clave_acceso);
    
                                        let dir = match fs::read_dir(d_dir) {
                                            Ok(dir) => dir,
                                            Err(_) => {
                                                println!("Error al leer el directorio.");
                                                continue;
                                            }
                                        };
    
                                        for dir_entry in dir {
                                            if let Ok(dir_data) = dir_entry {
                                                if let Ok(file_name) = dir_data.file_name().into_string() {
                                                    let full_dir_name = sanitize_name(&file_name);
            
                                                    let old_dir = PathBuf::from(d_dir).join(&file_name);
                                                    let new_dir = PathBuf::from(d_dir)
                                                        .join(&args.year)
                                                        .join(&args.user)
                                                        .join(&args.month)
                                                        .join(full_dir_name);
    
                                                        match fs::rename(&old_dir, &new_dir.join(format!("{}.xml", clave_acceso))) {
                                                            Ok(_) => {
                                                                println!("Archivo renombrado exitosamente: {} -> {}", clave_acceso, new_dir.display());
                                                                true_file = true;
                                                            }
                                                            Err(_) => {}
                                                        }
                                                }
                                            }
                                        }

                                        if true_file {
                                            break;
                                        }
                                    }
                                }
                            }

                        }
                    }
                }

                let footer_table = driver.find(By::Id("frmPrincipal:tablaCompRecibidos_paginator_bottom")).await?;
                let spans = footer_table.find_all(By::Tag("span")).await?;
                    
                for i in 0..spans.len() {
                    let text_span = spans[i].text().await?;
                    println!("{}", text_span);

                    if i == 5 {
                        println!("Si kbron si le hice click");
                        spans[i].click().await?;
                    }
                }

                // if !spans[3].wait_until().clickable().await.is_ok() {
                //     println!("El elemento no es clickeable")
                // }else {
                //     spans[3].click().await?;
                // }

            }
        }
        Err(_) => {
            println!("El captcha no se completó.");
        }
    }

    Ok(())
}

