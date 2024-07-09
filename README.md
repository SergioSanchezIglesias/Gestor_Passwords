# Requisitos
Esta aplicación necesita tener instalado ***sqlite***. Para instalarlo se siguen los siguiente pasos:
1. Descargar el comprimido que contiene *sqlite*: `https://www.sqlite.org/2024/sqlite-tools-win-x64-3460000.zip`.
2. Descomprimir en una carpeta de fácil acceso, por ejemplo `C\sqlite`.
3. Añadir la ruta donde se ha descomprimido a la variable de entorno ***Path***.

# Forma de uso
Esta aplicación puede correr de dos formas:
1. Generar un ejecutable y usarlo desde ahí: Basta con ejecutar el comando `cargo build --build` y despúes abrir el ejecutable `target/release/gestor_passwords.exe`.
2. Utilizar el comando `cargo run`.
   - *Utilizar este comando permite el paso de argumentos. utiliza `cargo run -- -h` para ver su forma de uso*.