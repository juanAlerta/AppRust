## Seguimiento TFG - Juan Blanco Martín
### 23/03/2022
---

En este documento voy a introducir brévemente el estado en el que llevo el TFG, así como 

### 1.- Arquitectura

El sistema sigue una especie de patrón ***modelo - vista - controlador***.


### 1.1.- Vista
La vista corresponde a la aplicación de escritorio que se ejecuta en una terminal, introduciendo los parámetros requeridos, de igual manera que otras herramientas del ámbito forense y de la ciberseguridad.

Está desarrollado en **Rust** y hace uso de la API **clap** (https://github.com/clap-rs/clap) que parsea los argumentos pasados y sus valores y estandariza este proceso.
