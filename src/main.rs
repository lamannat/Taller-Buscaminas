use std::env;
use std::fs;
mod matriz_buscaminas;

fn main() {
    let result_tablero_string = abrir_archivo();
    let tablero_string = match result_tablero_string {
        Ok(contenido_archivo) => contenido_archivo,
        Err(error) => {
            print!("No se pudo abrir el archivo: {}", error);
            return;
        }
    };
    let tablero_bytes = tablero_string.as_bytes();

    let mut tablero_matriz = matriz_buscaminas::MatrizBuscaminas::new();
    tablero_matriz.popular_desde_bytes(tablero_bytes);
    tablero_matriz.contar_bombas();
    tablero_matriz.imprimir_como_buscaminas();
}

/// Abre el archivo indicado como argumento al ejecutar el programa.
/// El argumento debe ser el path al archivo incluyendo el nombre del archivo
/// desde la carpeta raiz en la que se ejecute el programa.
///
/// Devuelve un Result<String, Error>, siendo el String el contenido del archivo y el Error el devuelto por fs::read_to_string.
///
/// #Ejemplo
///
/// ```
/// let contenido_result = abrir_archivo();
/// let contenido = match contenido_result {
///     Ok(contenido_archivo) => contenido_archivo,
///     Err(error) => {
///         print!("No se pudo abrir el archivo: {}", error);
///         return;
///     }
/// };
/// ```
fn abrir_archivo() -> Result<String, std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let archivo = &args[1];
    fs::read_to_string(archivo)
}
