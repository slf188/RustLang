// process nos ayuda a abortar o terminar el programa
use std::process;
use std::env;
// agregamos la structura Config en el archivo lib.rs
use minigrep::Config;

fn main(){
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // eprintln! es un macro que solo se utiliza para imprimir errores
        eprintln!("Problema con los argumentos: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
