/// Estructura que representa una matriz, con metodos específicos para ser utilizada
/// como tablero del juego buscaminas.
pub struct MatrizBuscaminas {
    valores: Vec<i32>,
    columnas: i32,
    filas: i32,
}

/// Constante que representa el valor del byte del caracter '*' en ASCII
const ASTERISCO_BYTE: u8 = b'*';

/// Constantes que representan el valor del byte del caracter '·' en ASCII
const INTERDOT_FIRST_BYTE: u8 = b'\xC2';
const INTERDOT_SECOND_BYTE: u8 = b'\xB7';
const DOT_BYTE: u8 = b'.';

impl MatrizBuscaminas {
    /// Devuelve una MatrizBuscaminas vacia.
    pub fn new() -> MatrizBuscaminas {
        MatrizBuscaminas {
            columnas: 0,
            filas: 0,
            valores: vec![0; 0],
        }
    }

    /// Metodo público que permite llenar una matriz vacia a partir de un arreglo
    /// de u8 compuesto por '*' representando bombas y '·' representando celdas vacias.
    /// Tambien genera una fila por cada salto de linea.
    ///
    /// # Argumentos
    ///
    /// * `bytes` - Arreglo de u8 con los caracteres que representan un juego de buscaminas.
    ///
    /// #Ejemplo
    ///
    /// ```
    /// let bytes = [b'*', b'.', b'*' , b'.', b'\n']
    /// let mut matriz = MatrizBuscaminas::new();
    /// matriz.popular_desde_bytes(bytes);
    /// ```
    pub fn popular_desde_bytes(&mut self, bytes: &[u8]) -> Result<(), String> {
        self.filas = Self::contar_filas(bytes);
        self.columnas = Self::contar_columnas(bytes);
        if !Self::validar_mapa(bytes, self.columnas) {
            return Err(
                "Mapa invalido, debe ser cuadrado o rectangular y estar compuesto por “·” o “*”"
                    .to_owned(),
            );
        }
        for byte in bytes {
            if *byte == ASTERISCO_BYTE {
                self.valores.push(-1)
            } else if *byte == INTERDOT_FIRST_BYTE || *byte == DOT_BYTE {
                self.valores.push(0)
            }
        }
        Ok(())
    }

    /// Metodo público que permite completar las celdas vacias de una MatrizBuscaminas con el número correspondiente
    /// segun la cantidad de bombas adyacentes.
    ///
    /// #Ejemplo
    ///
    /// ```
    /// matriz.contar_bombas();
    /// ```
    pub fn contar_bombas(&mut self) {
        for i in 0..self.valores.len() {
            if self.valores[i] == -1 {
                Self::aumentar_adyacentes(self, i as i32);
            }
        }
    }

    /// Metodo público que permite imprimir por salida estandar una MatrizBuscaminas
    /// con el formato del tablero de buscaminas.
    ///
    /// #Ejemplo
    ///
    /// ```
    /// let bytes = [b'*', b'.', b'*' , b'.', b'\n']
    /// let mut matriz = MatrizBuscaminas::new();
    /// matriz.popular_desde_bytes(&bytes);
    /// matriz.contar_bombas();
    /// matriz.imprimir_como_buscaminas();
    /// ```
    /// #Salida
    /// *2*1

    pub fn imprimir_como_buscaminas(&self) {
        let mut contador_columnas = 0;
        for valor in &self.valores {
            if contador_columnas == self.columnas {
                println!();
                contador_columnas = 0;
            }
            if *valor == -1 {
                print!("*");
            } else if *valor == 0 {
                print!("·");
            } else {
                print!("{}", *valor);
            }
            contador_columnas += 1;
        }
        println!();
    }

    /// Función interna del módulo.
    /// Cuenta la cantidad de columnas de un arreglo de u8 a partir de la
    /// cantidad de caracteres hasta el primer salto de linea.
    fn contar_columnas(bytes: &[u8]) -> i32 {
        let mut columnas = 0;
        for byte in bytes {
            if *byte == (b'\n') {
                break;
            } else if *byte == INTERDOT_FIRST_BYTE || *byte == ASTERISCO_BYTE || *byte == DOT_BYTE {
                columnas += 1;
            }
        }
        columnas
    }

    fn validar_mapa(bytes: &[u8], columnas: i32) -> bool {
        let mut contador = 0;
        for byte in bytes {
            if *byte == (b'\n') {
                if contador != columnas {
                    return false;
                }
                contador = 0;
            } else if *byte == INTERDOT_FIRST_BYTE || *byte == ASTERISCO_BYTE || *byte == DOT_BYTE {
                contador += 1;
            } else if *byte == INTERDOT_SECOND_BYTE || *byte == (b'\r') {
                continue;
            } else {
                return false;
            }
        }
        if contador != columnas {
            return false;
        }
        true
    }

    /// Función interna del módulo.
    /// Cuenta la cantidad de filas de un arreglo de u8 a partir de la
    /// cantidad de saltos de linea.
    fn contar_filas(bytes: &[u8]) -> i32 {
        let mut filas = 0;
        let mut contador = 1;
        for byte in bytes {
            if *byte == (b'\n') {
                filas += 1;
            }
            if contador == bytes.len() && *byte != (b'\n') {
                filas += 1;
                [bytes, &[b'\n']].concat();
            }
            contador += 1;
        }
        filas
    }

    /// Función interna del módulo.
    /// Aumenta en 1 el valor de las celdas adyacentes a una celda dada por el
    /// indice en el vector valores de la MatrizBuscaminas.
    fn aumentar_adyacentes(&mut self, i: i32) {
        let coord = Self::obtener_coordenadas(i, self.columnas);
        let celdas_a_aumentar = [
            (coord.0 - 1, coord.1 - 1),
            (coord.0 - 1, coord.1),
            (coord.0 - 1, coord.1 + 1),
            (coord.0, coord.1 - 1),
            (coord.0, coord.1 + 1),
            (coord.0 + 1, coord.1 - 1),
            (coord.0 + 1, coord.1),
            (coord.0 + 1, coord.1 + 1),
        ];
        for celda in celdas_a_aumentar {
            Self::aumentar_celda(self, celda);
        }
    }

    /// Función interna del módulo.
    /// Aumenta en 1 el valor de la celda indicada en caso que se encuentre dentro del
    /// tablero y no sea una celda con una mina.
    fn aumentar_celda(&mut self, celda: (i32, i32)) {
        if celda.0 < 0 || celda.0 >= self.filas || celda.1 < 0 || celda.1 >= self.columnas {
            return;
        }
        let posicion = celda.0 * self.columnas + celda.1;
        if self.valores[posicion as usize] != -1 {
            self.valores[posicion as usize] += 1;
        }
    }

    /// Función interna del módulo.
    /// Devuelve las coordenadas que tendria una celda en una representacion matricial
    /// a partir del indice en el vector valores de la MatrizBuscaminas.
    fn obtener_coordenadas(i: i32, col: i32) -> (i32, i32) {
        let fila = i / col;
        let columna = i % col;
        (fila, columna)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_crea_matriz_vacia() {
        let matriz = MatrizBuscaminas::new();
        assert_eq!(matriz.columnas, 0);
        assert_eq!(matriz.filas, 0);
        assert_eq!(matriz.valores.len(), 0);
    }

    #[test]
    fn test_popular_desde_bytes_agrega_menos_uno_si_encuentra_asterisco() {
        let mut matriz = MatrizBuscaminas::new();
        let bytes = [ASTERISCO_BYTE];
        match matriz.popular_desde_bytes(&bytes) {
            Ok(()) => {}
            Err(error) => {
                print!("Error al parsear archivo a tablero: {}", error);
                return;
            }
        }
        assert_eq!(matriz.valores.len(), 1);
        assert_eq!(matriz.valores[0], -1);
    }

    #[test]
    fn test_popular_desde_bytes_agrega_cero_si_encuentra_punto() {
        let mut matriz = MatrizBuscaminas::new();
        let bytes = [INTERDOT_FIRST_BYTE];
        match matriz.popular_desde_bytes(&bytes) {
            Ok(()) => {}
            Err(error) => {
                print!("Error al parsear archivo a tablero: {}", error);
                return;
            }
        }
        assert_eq!(matriz.valores.len(), 1);
        assert_eq!(matriz.valores[0], 0);
    }

    #[test]
    fn test_popular_desde_bytes_agrega_fila_si_encuentra_salto_de_linea() {
        let mut matriz = MatrizBuscaminas::new();
        let bytes = [b'\n', b'\n', b'\n'];
        match matriz.popular_desde_bytes(&bytes) {
            Ok(()) => {}
            Err(error) => {
                print!("Error al parsear archivo a tablero: {}", error);
                return;
            }
        }
        assert_eq!(matriz.valores.len(), 0);
        assert_eq!(matriz.filas, 3);
    }

    #[test]
    fn test_popular_desde_bytes_agrega_columnas_si_no_encuentra_salto_de_linea() {
        let mut matriz = MatrizBuscaminas::new();
        let bytes = [INTERDOT_FIRST_BYTE, INTERDOT_FIRST_BYTE];
        match matriz.popular_desde_bytes(&bytes) {
            Ok(()) => {}
            Err(error) => {
                print!("Error al parsear archivo a tablero: {}", error);
                return;
            }
        }
        assert_eq!(matriz.valores.len(), 2);
        assert_eq!(matriz.columnas, 2);
    }

    #[test]
    fn test_contar_bombas_suma_adyacentes_horizontales() {
        let mut matriz = MatrizBuscaminas::new();
        let bytes = [
            INTERDOT_FIRST_BYTE,
            ASTERISCO_BYTE,
            INTERDOT_FIRST_BYTE,
            b'\n',
        ];
        match matriz.popular_desde_bytes(&bytes) {
            Ok(()) => {}
            Err(error) => {
                print!("Error al parsear archivo a tablero: {}", error);
                return;
            }
        }
        matriz.contar_bombas();
        assert_eq!(matriz.valores[0], 1);
        assert_eq!(matriz.valores[2], 1);
    }

    #[test]
    fn test_contar_bombas_suma_adyacentes_verticales() {
        let mut matriz = MatrizBuscaminas::new();
        let bytes = [
            INTERDOT_FIRST_BYTE,
            b'\n',
            ASTERISCO_BYTE,
            b'\n',
            INTERDOT_FIRST_BYTE,
            b'\n',
        ];
        match matriz.popular_desde_bytes(&bytes) {
            Ok(()) => {}
            Err(error) => {
                print!("Error al parsear archivo a tablero: {}", error);
                return;
            }
        }
        matriz.contar_bombas();
        assert_eq!(matriz.valores[0], 1);
        assert_eq!(matriz.valores[2], 1);
    }

    #[test]
    fn test_contar_bombas_suma_adyacentes_diagonales() {
        let mut matriz = MatrizBuscaminas::new();
        let bytes = [
            INTERDOT_FIRST_BYTE,
            INTERDOT_FIRST_BYTE,
            b'\n',
            ASTERISCO_BYTE,
            INTERDOT_FIRST_BYTE,
            b'\n',
            INTERDOT_FIRST_BYTE,
            INTERDOT_FIRST_BYTE,
            b'\n',
        ];
        match matriz.popular_desde_bytes(&bytes) {
            Ok(()) => {}
            Err(error) => {
                print!("Error al parsear archivo a tablero: {}", error);
                return;
            }
        }
        matriz.contar_bombas();
        assert_eq!(matriz.valores[1], 1);
        assert_eq!(matriz.valores[5], 1);
    }

    #[test]
    fn test_contar_bombas_suma_corectamente_dos_bombas() {
        let mut matriz = MatrizBuscaminas::new();
        let bytes = [ASTERISCO_BYTE, INTERDOT_FIRST_BYTE, ASTERISCO_BYTE, b'\n'];
        match matriz.popular_desde_bytes(&bytes) {
            Ok(()) => {}
            Err(error) => {
                print!("Error al parsear archivo a tablero: {}", error);
                return;
            }
        }
        matriz.contar_bombas();
        assert_eq!(matriz.valores[1], 2);
    }

    #[test]
    fn test_contar_bombas_suma_corectamente_tres_bombas() {
        let mut matriz = MatrizBuscaminas::new();
        let bytes = [
            ASTERISCO_BYTE,
            INTERDOT_FIRST_BYTE,
            ASTERISCO_BYTE,
            b'\n',
            INTERDOT_FIRST_BYTE,
            ASTERISCO_BYTE,
            INTERDOT_FIRST_BYTE,
        ];
        match matriz.popular_desde_bytes(&bytes) {
            Ok(()) => {}
            Err(error) => {
                print!("Error al parsear archivo a tablero: {}", error);
                return;
            }
        }
        matriz.contar_bombas();
        assert_eq!(matriz.valores[1], 3);
    }
}
