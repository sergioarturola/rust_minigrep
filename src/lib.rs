use std::error::Error; //Box dyn
use std::fs;

//con la estructura tenemos los datos almacenados y no como "variables sueltas"
pub struct  Config{
    pub consulta: String,
    pub ruta_archivo: String,
}

//implementando la funcion "build" para la estructura
impl Config{
    /* 
    Esta funcion no ayudaba en el manejo de errores
    fn new(args: &[String]) -> Config{
        let consulta = args[1].clone();
        let ruta_archivo = args[2].clone();
    
        Config {consulta, ruta_archivo}
    }*/

    //funcion encargada de retornar una estructura y parsear los argumentos pasados x consola
    pub fn build(args: &[String]) -> Result<Config, &'static str>{
        //si se pasaron menos de 3 argumentos
        if args.len() < 3{
            return Err("No son suficientes argumentos");//este es el string estatico para el error
        }
        let consulta = args[1].clone();
        let ruta_archivo = args[2].clone();
    
        Ok(Config {consulta, ruta_archivo})

    }
}

//funcion que se encarga de leer el archivo pasado por linea de comandos, ahora que ya no esta
//en main la logica se pueden manejar los errores, el error es una "interfaz" y "dyn" es un 
//error dinamico, ademas la funcion no retorna nada ()
pub fn run(configuracion: Config)-> Result<(), Box<dyn Error>>{//instancia de la estructura Config
    //leyendo el contenido
    let contenido = fs::read_to_string(configuracion.ruta_archivo)?;//ya no usa except ahora ?
   
    println!("El texto:\n{contenido}");
    //si todo sale bien, el ok va vacio
    Ok(())
}

//funcion de busqueda
pub fn search<'a>(query: &str, contents: &'a str)->Vec<&'a str>{
    let mut result = Vec::new();

    for line in contents.lines(){//iterando por cada linea
        if line.contains(query) {
            result.push(line);
        }

    }
    result
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn un_resultado(){
        let consulta = "duct";
        let contenido = "/
Rust:
safe, fast, productive.
Pick Three";

        assert_eq!(vec!["safe, fast, productive."], search(consulta, contenido));
    }
}