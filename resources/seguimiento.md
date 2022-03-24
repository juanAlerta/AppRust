## Seguimiento TFG - Juan Blanco Martín
### 23/03/2022
---

En este documento voy a introducir brévemente el estado en el que llevo el TFG.

**Tooth** es un sistema forense que registra el software que se ejecuta en un ordenador, así como información sobre esta ejecución, como el momento en el que se ejecuta o el usuario sobre el que lo hace. Continuamnete va escribiendo la actividad en un **.log** que podrá analizarse para ver lo ocurrido.

### 1.- Arquitectura

El sistema sigue una especie de patrón ***modelo - vista - controlador***.


### 1.1.- Vista
La vista corresponde a la aplicación de escritorio que se ejecuta en una terminal, introduciendo los parámetros requeridos, de igual manera que otras herramientas del ámbito forense y de la ciberseguridad.

Está desarrollado en **Rust** y hace uso de la API **clap** (https://github.com/clap-rs/clap) que parsea los argumentos pasados y sus valores y estandariza este proceso. 

La estructura básica de la aplicación ya está hecha, con los comandos básicos para poder probarse con el resto de subsistemas según se vayan desarrollando. Estos son: 
- time n --> requiere un número entero. El sistema realizará el registro durante n minutos.
- notime --> inicia el registro sin límite de tiempo hasta que el usuario lo pare.
- info --> devuelve información general como la ubicación en la que se encuentra el fichero de registro.

Como subargumentos estarían:
- dir d --> requiere un string. Define la ubicación d en la que se ubicará el fichero de registro en lugar de la ubicación redeterminada.
- show --> permite ver en la terminal el registro en tiempo real.

En la implementación actual los subcomandos aún no están.

```rust
use std::{env, num::ParseIntError};
use clap::{Arg, Command, arg};

// de momento no uso los enum
enum Argument_primary_type {
    Time,
    Notime,
    Info
}
enum Argument_secundary_type {
    Dir,
    Show,
    Zero
}

struct Argument {
    primary_type: Argument_primary_type,
    secundary_type: Argument_secundary_type,
    extra_type: Argument_secundary_type,
    time_atr: u16,
    dir_atr: String,
}

fn main() {

    

    let matches = Command::new("tooth")
        .about("Tooth is a forensic computer analysis wich notes the programs running on your operative system.\nDocumentation: https://github.com/juanAlerta/AppRust")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .allow_invalid_utf8_for_external_subcommands(true)
        .subcommand(
            // time
            Command::new("time")
                .short_flag('t')
                .about("Define the time that the application will be doing the registry, or use <notime>")
                .arg(arg!(<UNIDAD_TIEMPO> "The time required"))
                .arg_required_else_help(true)
                /* .arg(
                Arg::new("dir")
                    .short('d')
                    .help("Define the file path.")
                    .arg(arg!(<RUTA> "Direction"))
                    .takes_value(true),
                ) */
        )
        .subcommand(
            Command::new("notime")
                .short_flag('n')
                .about("Start registry with no time limit"), 
        )
        .subcommand(
            Command::new("info")
                .short_flag('i')
                .about("show general info")
        )
        .get_matches();
    
    match matches.subcommand() {
        Some(("time", time_matches)) => {
            let unidad_tiempo:u16 = time_matches.value_of("UNIDAD_TIEMPO").expect("required").parse().unwrap();
            // validar tipo
            println!("✍️ Starting registry for {} seconds along ✍️", time_matches.value_of("UNIDAD_TIEMPO").expect("required"));

            /*
            if time_matches.is_present("dir") {
                let direccion: Vec<_> = time_matches.values_of("dir").unwrap().collect();
                let values = direccion.join(", ");
                println!("🤓 Saving data in {} 🤓", values);
                return;
            }*/ 
        }

        Some(("notime", cosa)) => { //cosa porque me pide una tupla, pero no hace nada
            println!("✍️ Starting registry with no time limit ✍️");
        }

        Some(("info", cosa)) => { //cosa porque me pide una tupla, pero no hace nada
            println!("<INFORMACION GENERAL>");
        }
        
        _ => unreachable!(), 
    }
}  
```

Cuando se introducen los parámetros apropiados la **vista** devuelve al **controlador** un objeto con los requerimientos del usuario para realizar el registro, aunque aún no está implementado. 

### 1.1.- Controlador
