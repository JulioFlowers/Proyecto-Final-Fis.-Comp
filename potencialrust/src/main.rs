
/*La idea general de codigo es que resuelve la ecuación de Laplace utilizando  diferencias 
centradas de cuarto orden, para ello se genera una matriz de 2046x430 donde cada indice 
representa 1 um y se rellena con un valor arbitrario (en este caso la mitad del potencial 
más grande)

https://www.dam.brown.edu/people/alcyew/handouts/numdiff.pdf

Posteriormente se proponen las fronteras de tipo Dirichlet mutando los valores de algunas zonas
de la matriz con respecto al potencial de nuestro electrodos 5 y 0 Volts y a la geometria de 
nuestros electrodos (ver PDF).

Finalmente se busca el conjunto de pares ordenados donde los valores con los que se relleno la 
matriz en un principio no fueron mutados. Esto es importante porque, debido a que el potencial 
de nuestra frontera no debe ser mutado por la diferencia finita entonces solo iteramos en esa 
colección de indices y con eso ahorramos tiempo por cada paso que hace 

En  otro proceso aparte exporta los datos de la matriz cuando ya se terminaron de hacer 10000
iteraciones en un archivo txt con escritura paralela (para no demorarse tanto tiempo) 
para poder graficar en python porque no sabemos hacerlo en Rust.*/



use rayon::prelude::*; //libreria de escritura paralela
use std::fs::File; //elemento de la libreria estandar de Rust que accesa rutas del ordenado 
use std::io::{self, Write}; //metodo escribir sobre objeto de la libreria estandar de Rust

//numero maximo de iteraciones 
const MAX_ITER: usize = 10000;


//Declaración de nuestras funciones
//-------------------------------------------------------------------------------------------------

/*  Función para crear una matriz de m filas por n columnas con todos sus elementos valiendo 1.75.
 Este constructor usa la estructura vector de vectores */

fn crear_matriz(m: usize, n: usize) -> Vec<Vec<f32>> {
    let mut matriz = Vec::with_capacity(m);

    for _ in 0..m {
        let fila: Vec<f32> = vec![1.75; n];
        matriz.push(fila);
    }

    matriz
}



/*En esta funcion lo que se hace es realizar dos iteraciones sobre una matriz mutables y obtener sus 
respectivos indices i,j si el valor del elemento matriz[i][j] es igual a un valor buscado, nos regresa
un vector con una colección de pares ordenados.*/

fn obtener_valores(matriz: &mut Vec<Vec<f32>>, valor_buscado: f32) -> Vec<(usize, usize)> {
    let mut indices_a_cambiar = Vec::new();

    for (i, fila) in matriz.iter_mut().enumerate() {
        for (j, valor) in fila.iter_mut().enumerate() {
            if *valor == valor_buscado {
                // Guardar los índices
                indices_a_cambiar.push((i, j));
            }
        }
    }

    indices_a_cambiar
}



/*Esta descripción sale directa de ChatGPT, porque le pedi esta funcion 

Esta función, llamada write_matrix_to_file_parallel, toma una matriz mutable de números de punto flotante (f32) 
y un camino de archivo (file_path), y escribe el contenido de la matriz en un archivo. Aquí está el desglose de la función:

matrix.par_iter_mut(): Esto utiliza la función par_iter_mut() proporcionada por la biblioteca rayon para realizar una iteración  
paralela sobre las filas de la matriz. La iteración paralela permite procesar múltiples filas simultáneamente, lo que puede mejorar 
el rendimiento en sistemas con varios núcleos de CPU.

.map(|row| { ... }): Para cada fila de la matriz, se ejecuta una función de mapeo que realiza las siguientes operaciones:

row.iter_mut().map(|num| num.to_string()): Itera sobre los números de la fila y los convierte a cadenas de texto. Esto se hace 
utilizando iter_mut() para obtener iteradores mutables sobre los elementos de la fila y map(|num| num.to_string()) para convertir cada
número a su representación de cadena.

.collect::<Vec<String>>(): Recoge los resultados de la operación de mapeo en un vector de cadenas.

.join("\t"): Utiliza el método join para unir las cadenas en una sola cadena, separando cada número por una tabulación ("\t").

.collect(): Recoge los resultados de la iteración paralela en un vector de cadenas llamado formatted_rows.

let mut file = File::create(file_path)?;: Abre un nuevo archivo en el camino especificado utilizando File::create(). 
El operador ? se utiliza para manejar automáticamente los errores y devolver un error si la operación de apertura del archivo no tiene éxito.

for row_str in formatted_rows { ... }: Itera sobre cada cadena formateada en formatted_rows.

writeln!(file, "{}", row_str)?;: Escribe la cadena en el archivo seguido de un salto de línea utilizando writeln!.
 Nuevamente, el operador ? se utiliza para manejar automáticamente los errores.
Ok(()): Después de escribir todas las filas en el archivo, la función devuelve Ok(()), indicando que la operación se realizó con éxito.

En resumen, esta función utiliza la programación paralela para formatear y escribir las filas de una matriz en un archivo de manera eficiente, 
aprovechando la capacidad de procesamiento paralelo para mejorar el rendimiento en sistemas con múltiples núcleos de CPU.

*/

fn write_matrix_to_file_parallel(matrix: &mut Vec<Vec<f32>>, file_path: &str) -> io::Result<()> {
    let formatted_rows: Vec<String> = matrix
        .par_iter_mut()
        .map(|row| {
            // Formatear la fila como una cadena separada por tabulaciones
            row.iter_mut()
                .map(|num| num.to_string())
                .collect::<Vec<String>>()
                .join("\t")
        })
        .collect();

    let mut file = File::create(file_path)?;

    for row_str in formatted_rows {
        // Escribir la fila en el archivo seguido de un salto de línea
        writeln!(file, "{}", row_str)?;
    }

    Ok(())
}
//-------------------------------------------------------------------------------------------------


//Ejecución Principal.
//-------------------------------------------------------------------------------------------------
fn main()
{
    // Número de filas y columnas

    /*Cabe aclarar que se agregaron cuatro puntos mas para satisfacer las condiciones iniciales del
    metodo de diferencias finitas centradas de cuarto orden y no perder información, i,e i<=2, e
     i<= n-2, de tal forma que el termino i+2 corresponde al 0 de nuestro sistema real */
    let m = 2046;
    let n = 2430;

    // Crear la matriz

    let mut phi: Vec<Vec<f32>> = crear_matriz(m, n);


    /* En ambos casos (potencial positivo y potencial 0),lo que se hace es iterar sobre las filas primero
    y despues sobre las columnas. si el indice de las columnas se encuentra en una de las colecciones de 
    puntos mencionadas (las cuales corresponden geometricamente a los electrodos del capacitor coplanar),
    entonces modifica el valor de la matriz al del potencial deseado.*/


    //potencial positivo
    for i in 2..m - 2 {
        if (0..=194).contains(&i) {
            for j in 0..n {
                phi[i][j] = 3.5;
            }
        }

        if (194..=704).contains(&i) {
            for range in [(0..385), (1021..1404), (2040..n)] {
                for j in range {
                    phi[i][j] = 3.5;
                }
            }
        }

        if (704..=1340).contains(&i) {
            for range in [(0..385), (2040..n)] {
                for j in range {
                    phi[i][j] = 3.5;
                }
            }
        }

        if (1340..=1723).contains(&i) {
            for range in [(0..898), (1535..n)] {
                for j in range {
                    phi[i][j] = 3.5;
                }
            }
        }
    }

    //potencial 0
    for i in 0..m {
        if (322..=832).contains(&i) {
            for range in [(512..894), (1531..1913)] {
                for j in range {
                    phi[i][j] = 0.0;
                }
            }
        }

        if (832..=1214).contains(&i) {
            for j in 512..1913 {
                phi[i][j] = 0.0;
            }
        }

        if (1214..=1851).contains(&i) {
            for j in 1022..1405 {
                phi[i][j] = 0.0;
            }
        }

        if i >= 1851 {
            for j in 0..n {
                phi[i][j] = 0.0;
            }
        }
    }

    //obtener indices a iterar
    let indices = obtener_valores(&mut phi, 1.75);

    //iteraciones del metodo
    for _k in 0..MAX_ITER {

        /* Como la coleccion de parejas ordenadas que obtuvimos en la sección anterior no es
        accesible para operar en este ciclo las clonamos (supongo que hay una forma mas optima
        de pasarlas al scope) pero se itera sobre esa colección de puntos 
        
        Sabemos que en esa colección de puntos hay puntos que no cumplen las condiciones iniciales
        ie i<=2, e i<= n-2, entonces forazmos a que solo itere sobre los puntos que si cumplen esa 
        condición y evalue el nuevo valor de phi (checar foto del desgloze de la solución de la 
        ecuación de laplace en la foto))*/

        for (i, j) in indices.clone() {
            // Verificar que los índices estén dentro del rango de la matriz
            if i < phi.len() && j < phi[0].len() {
                if i >= 2 && j >= 2 && i + 2 < phi.len() && j + 2 < phi[0].len() {
                    phi[i][j] = (16.0 * phi[i - 1][j]
                        + 16.0 * phi[i][j - 1]
                        + 16.0 * phi[i + 1][j]
                        + 16.0 * phi[i][j + 1]
                        - 1.0 * phi[i - 2][j]
                        - 1.0 * phi[i][j - 2]
                        - 1.0 * phi[i + 2][j]
                        - 1.0 * phi[i][j + 2])
                        / 60.0;
                }
            } 
        }

        println!("Iteración n°: {}.", _k);
    }

    /*hacemos las matrices de las componentes en x y Y del campo electrico de nuevo de la matriz de potencial hay 
    cuatro puntos mas para satisfacer las condiciones iniciales del  metodo de diferencias finitas centradas de cuarto 
    orden pero no nos importan y los quitamos de las matrices resultantes
    */

    let mut ex: Vec<Vec<f32>> = crear_matriz(m - 4, n - 4); // Componente x del campo eléctrico
    let mut ey: Vec<Vec<f32>> = crear_matriz(m - 4, n - 4); // Componente y del campo eléctrico

    /*Iteramos sobre las matrices Ey Ex y Phi para aplicar el operador numerico gradiente el cual es diferencia finita
     centrada de cuarto orden para los elementos de x dejando a y fija y la diferencia finita centrada de cuarto orden para 
     los elementos de y dejando a x fija.
     
     los indices ip e ij estan correjidos para poder acceder phi y sus condiciones iniciales*/

    for i in 0..m - 4 {
        let ip = i + 2;
        for j in 0..n - 4 {
            let jp = j + 2;
            ey[i][j] = -1.0
                * (-1.0 * phi[ip + 2][jp] + 16.0 * phi[ip + 1][jp] - 30.0 * phi[ip][jp]
                    + 16.0 * phi[ip - 1][jp]
                    - 1.0 * phi[ip - 2][jp])
                / 12.0;
            ex[i][j] = -1.0
                * (-1.0 * phi[ip][jp + 2] + 16.0 * phi[ip][jp + 1] - 30.0 * phi[ip][jp]
                    + 16.0 * phi[ip][jp - 1]
                    - 1.0 * phi[i][jp - 2])
                / 12.0;
        }
    }

    let mut rho: Vec<Vec<f32>> = crear_matriz(m - 6, n - 6);

    for i in 0..m - 6 {
        for j in 0..n - 6 {
            // Asegúramos que los índices estén dentro de los límites antes de acceder a rho
            if i + 1 < rho.len() && i >= 1 && j + 1 < rho[0].len() && j >= 1 {
                rho[i][j] = (ey[i + 1][j] - ey[i - 1][j]) + (ex[i][j + 1] - ex[i][j - 1]);
            } 
        }
    }
    

    // Llamar a la función para escribir en el archivo de manera paralela y checar si no hay error
    if let Err(e) = write_matrix_to_file_parallel(&mut phi, "output.txt") {
        eprintln!("Error al escribir en el archivo: {}", e);
    } else {
        println!("Datos escritos exitosamente en el archivo.");
    }

    if let Err(e) = write_matrix_to_file_parallel(&mut ex, "ex.txt") {
        eprintln!("Error al escribir en el archivo: {}", e);
    } else {
        println!("Datos escritos exitosamente en el archivo.");
    }

    if let Err(e) = write_matrix_to_file_parallel(&mut ey, "ey.txt") {
        eprintln!("Error al escribir en el archivo: {}", e);
    } else {
        println!("Datos escritos exitosamente en el archivo.");
    }

    if let Err(e) = write_matrix_to_file_parallel(&mut rho, "carga.txt") {
        eprintln!("Error al escribir en el archivo: {}", e);
    } else {
        println!("Datos escritos exitosamente en el archivo.");
    }
}


