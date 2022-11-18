// cambiar el nombre del proyecto
// retornar errores en tipo String
use std::error::Error;
// copia contenidos de un archivo a otro
use std::fs;
// nos ayuda a procesar argumentos
use std::env;

pub struct Config {
    pub busqueda: String,
    pub archivo: String,
}

impl Config {
    // el parametro args el argumento pasado por el usuario de la busqueda que solicita hacer
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        // next lo que hace es para agarrar el siguiente valor de la lista de argumentos
        // match es como un if pero mas poderoso
        // match nos ayuda a determinar si tenemos un argumento o no
        let busqueda = match args.next() {
            Some(arg) => arg,
            // si no hay argumentos, significa que el usuario no busca nada
            None => return Err("No se consiguio el argumento de busqueda"),
        };
        // el segundo argumento es el archivo en el que se va a buscar
        // utilizamos match para retornar el archivo o imprimir un error que no se consiguio el archivo
        let archivo = match args.next() {
            Some(arg) => arg,
            None => return Err("No se consiguio el argumento de archivo"),
        };

        Ok(Config { busqueda, archivo })
    }
}

// aqui procesamos la instancia de Config
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // aqui leemos los contenidos del archivo, ? es un operador de errores
    let contents = fs::read_to_string(config.archivo)?;
    // ya teniendo los contenidos del archivo, los procesamos la busqueda
    let results = { search(&config.busqueda, &contents) };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

// aqui separamos el argumento de busqueda
pub fn search<'a>(busqueda: &str, contents: &'a str) -> Vec<&'a str> {
    // filter lo que hace es iterar sobre el contenido del archivo
    // contains lo que hace es buscar el argumento de busqueda en el contenido del archivo
    contents
        .lines()
        .filter(|line| line.contains(busqueda))
        .collect()
}
