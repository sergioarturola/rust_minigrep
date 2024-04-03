use std::env;//para recolectar los argumentos de la linea de comandos
use std::process;//para retornar 1
use proyecto_grep::Config;

fn main() {
    //args es duseño del texto y el nombre del archivo
    let args: Vec<String> = env::args().collect();

    /*
     Manejando los errores, en caso de que Result retorne "Err" entonces se manda
     mensaje + mensaje de unrwap en caso contrario se retorna el Result por eso es
     unwrap_or_else es como decirle "desenvuelve" el resultado y sino ejecuta lo
     que esta en la funcion anonima
    */
    let configuracion = Config::build(&args).unwrap_or_else(|err|{
        //en caso de que entre en error se mostrara el mensaje y se retorna 1
        println!("Problemas al parsear los argumentos: {err}");
        process::exit(1);

    });

    println!("Buscando por {}", configuracion.consulta);
    println!("En el archivo {}", configuracion.ruta_archivo);

    //para que el compilador no reclame de que pudo haber un error en la funcion (por el result)
    //que no se haya checado es necesario verificar y se hara con un if
    if let Err(e) = proyecto_grep::run(configuracion){
        println!("Error en la aplicacion {e}");
        process::exit(1)
        //asi la advertencia ya se va, AQUI SOLO IMPORTA DETECTAR UN ERROR POR ESO EL
        //if y ademas NO SE USO unwrap_or_else porque solo se desea detectar el error
    }

     
}







