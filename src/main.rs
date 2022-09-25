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
    match tablero_matriz.popular_desde_bytes(tablero_bytes) {
        Ok(()) => {}
        Err(error) => {
            print!("Error al parsear archivo a tablero: {}", error);
            return;
        }
    }
    tablero_matriz.contar_bombas();
    tablero_matriz.imprimir_como_buscaminas();
}

/// Abre el archivo indicado como argumento al ejecutar el programa.
/// El argumento debe ser el path al archivo incluyendo el nombre del archivo
/// desde la carpeta raiz en la que se ejecute el programa.
///
/// Devuelve un Result<String, Error>, siendo el String el contenido del archivo y Error en caso de que la cantidad de argumentos de entrada no sea la correcta, o
/// el error devuelto por fs::read_to_string si no se puede leer el archivo.
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
    if args.len() != 2 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Cantidad de argumentos invalida",
        ));
    }
    let archivo = &args[1];
    fs::read_to_string(archivo)
}
