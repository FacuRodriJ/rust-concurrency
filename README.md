# Descripción del Proyecto

Este es un programa en Rust que utiliza múltiples hilos para descargar datos de una API del gobierno argentino que proporciona información sobre municipios, departamentos y localidades. Los datos son guardados en formato JSON y CSV.

## Requisitos

Para ejecutar este programa, es necesario tener instalados los siguientes requisitos:

- Visual Studio Code (Opcional)
- Docker
- Docker Compose

## Preparación del Entorno

1. Clonar este repositorio: `git clone https://github.com/FacuRodriJ/rust-concurrency.git`
2. Abrir el proyecto en Visual Studio Code: `cd rust-concurrency && code .`

## Caso Práctico

Para descargar los datos de la API, se utilizan las siguientes librerías de Rust:

- reqwest: para hacer las peticiones HTTP.
- serde: para la serialización y deserialización de los datos.
- serde_json: para la manipulación de datos en formato JSON.
- csv: para guardar los datos en formato CSV.
- tokio: para manejar múltiples hilos de ejecución.

## Uso

Para ejecutar el programa, es necesario estar dentro del contenedor de Docker:

1. Ejecutar el comando `docker-compose run dev bash` dentro del directorio del proyecto para entrar al contenedor.
2. Moverse al directorio del proyecto: `cd /rust_app/app`.
3. Ejecutar el comando `cargo run` para correr el programa.

Los archivos con los datos descargados se guardarán en la carpeta `data`.
