/*
El programa es un script en el lenguaje de programación Rust que utiliza múltiples hilos para descargar datos 
de una API del gobierno argentino que proporciona información sobre municipios, departamentos y localidades.

Primero, el script crea un directorio llamado "data" si no existe y, luego, utiliza tres hilos diferentes para 
descargar los datos de los municipios, departamentos y localidades en paralelo.

Cada hilo ejecuta una función llamada get_municipios(), get_departamentos() y get_localidades(), respectivamente. 
Cada una de estas funciones utiliza la biblioteca reqwest para hacer una solicitud HTTP a una URL de API específica 
y, luego, escribe la respuesta JSON en un archivo dentro del directorio "data". Luego, las funciones analizan la 
respuesta JSON en un objeto Value utilizando la biblioteca serde_json, y escriben los datos relevantes en un archivo 
CSV utilizando la biblioteca csv.

El programa finaliza después de que todos los hilos hayan completado sus tareas, lo que se logra con la función join() 
de Rust, que espera a que un hilo termine antes de continuar con la ejecución.
*/

extern crate reqwest;
extern crate csv;

use std::fs::File;
use std::io::prelude::*;
use serde_json::Value;
use std::path::Path;
use std::thread;

fn main() {
    let path = Path::new("data");
    if !path.exists() {
        std::fs::create_dir_all(&path).unwrap();
    }
    
    let municipios_handle = thread::spawn(|| {
        println!("Descargando municipios... Hilo: {:?}", thread::current().id());
        match get_municipios("https://apis.datos.gob.ar/georef/api/municipios?max=5000") {
            Ok(_) => println!("Municipios descargados correctamente"),
            Err(e) => println!("Error: {}", e),
        }
    });

    let departamentos_handle = thread::spawn(|| {
        println!("Descargando departamentos... Hilo: {:?}", thread::current().id());
        match get_departamentos("https://apis.datos.gob.ar/georef/api/departamentos?max=5000") {
            Ok(_) => println!("Departamentos descargados correctamente"),
            Err(e) => println!("Error: {}", e),
        }
    });

    let localidades_handle = thread::spawn(|| {
        println!("Descargando localidades... Hilo: {:?}", thread::current().id());
        match get_localidades("https://apis.datos.gob.ar/georef/api/localidades?max=5000") {
            Ok(_) => println!("Localidades descargadas correctamente"),
            Err(e) => println!("Error: {}", e),
        }
    });

    println!("Hilo principal: {:?}", thread::current().id());
    

    // Join the threads to wait for them to finish
    municipios_handle.join().unwrap();
    departamentos_handle.join().unwrap();
    localidades_handle.join().unwrap();
}

#[tokio::main]
async fn get_municipios(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Fetch the JSON data from the URL
    let response = reqwest::get(url).await?;
    let body = response.text().await?;

    // Write the JSON data to a file
    let mut json_file = File::create("data/municipios.json")?;
    json_file.write_all(body.as_bytes())?;

    // Parse the JSON data into a Value struct
    let municipios: Value = serde_json::from_str(&body)?;

    // Create a CSV writer to write the CSV data to a file
    let mut wtr = csv::Writer::from_writer(File::create("data/municipios.csv")?);

    // Write the header row
    wtr.write_record(&["id", "Nombre", "Provincia", "Lat", "Lon"])?;

    // Loop over all the municipios in the JSON data
    for municipio in municipios["municipios"].as_array().unwrap() {
        // Write the data for this municipio to the CSV file
        wtr.write_record(&[
            municipio["id"].as_str().unwrap_or(""),
            municipio["nombre"].as_str().unwrap_or(""),
            municipio["provincia"]["nombre"].as_str().unwrap_or(""),
            municipio["centroide"]["lat"].as_f64().unwrap_or(0.0).to_string().as_str(),
            municipio["centroide"]["lon"].as_f64().unwrap_or(0.0).to_string().as_str(),
        ])?;
    }

    // Flush the CSV writer to make sure all records are written
    wtr.flush()?;
    Ok(())
}   

#[tokio::main]
async fn get_departamentos(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Fetch the JSON data from the URL
    let response = reqwest::get(url).await?;
    let body = response.text().await?;

    // Write the JSON data to a file
    let mut json_file = File::create("data/departamentos.json")?;
    json_file.write_all(body.as_bytes())?;

    // Parse the JSON data into a Value struct
    let departamentos: Value = serde_json::from_str(&body)?;

    // Create a CSV writer to write the CSV data to a file
    let mut wtr = csv::Writer::from_writer(File::create("data/departamentos.csv")?);

    // Write the header row
    wtr.write_record(&["id", "Nombre", "Provincia", "Lat", "Lon"])?;

    // Loop over all the departamentos in the JSON data
    for departamento in departamentos["departamentos"].as_array().unwrap() {
        wtr.write_record(&[
            departamento["id"].as_str().unwrap_or(""),
            departamento["nombre"].as_str().unwrap_or(""),
            departamento["provincia"]["nombre"].as_str().unwrap_or(""),
            departamento["centroide"]["lat"].as_f64().unwrap_or(0.0).to_string().as_str(),
            departamento["centroide"]["lon"].as_f64().unwrap_or(0.0).to_string().as_str(),
        ])?;
    }

    // Flush the CSV writer to make sure all records are written
    wtr.flush()?;
    Ok(())
}

#[tokio::main]
async fn get_localidades(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Fetch the JSON data from the URL
    let response = reqwest::get(url).await?;
    let body = response.text().await?;

    // Write the JSON data to a file
    let mut json_file = File::create("data/localidades.json")?;
    json_file.write_all(body.as_bytes())?;

    // Parse the JSON data into a Value struct
    let localidades: Value = serde_json::from_str(&body)?;

    // Create a CSV writer to write the CSV data to a file
    let mut wtr = csv::Writer::from_writer(File::create("data/localidades.csv")?);

    // Write the header row
    wtr.write_record(&["id", "Nombre", "Categoria", "Departamento", "Municipio", "Provincia", "Lat", "Lon"])?;
    
    // Loop over all the localidades in the JSON data
    for localidad in localidades["localidades"].as_array().unwrap() {
        wtr.write_record(&[
            localidad["id"].as_str().unwrap_or(""),
            localidad["nombre"].as_str().unwrap_or(""),
            localidad["categoria"].as_str().unwrap_or(""),
            localidad["departamento"]["nombre"].as_str().unwrap_or(""),
            localidad["municipio"]["nombre"].as_str().unwrap_or(""),
            localidad["provincia"]["nombre"].as_str().unwrap_or(""),
            localidad["centroide"]["lat"].as_f64().unwrap_or(0.0).to_string().as_str(),
            localidad["centroide"]["lon"].as_f64().unwrap_or(0.0).to_string().as_str(),
        ])?;
    }

    // Flush the CSV writer to make sure all records are written
    wtr.flush()?;
    Ok(())
}
