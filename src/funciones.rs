use std::io::{self, Write};

use crate::db;
use crate::usuario;
use crate::usuario::UserManager;
use crate::usuario::Usuario;

pub fn show_menu() {
    println!("\n1. Agregar usuario");
    println!("2. Autenticar usuario");
    println!("3. Salir");
}

pub fn show_help() {
    println!("Uso:");
    println!("cargo run -- -h --help     Mostrar ayuda.");
    println!("cargo run -- <username> <password>     Entra a la aplicación estando autenticado.");
    println!("cargo run     Entra a la aplicación sin autenticar.")
}

pub fn get_option() -> u32 {
    loop {
        let mut choice: String = String::new();

        println!("Seleccione una opción");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line.");

        match choice.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Introduce un número válido."),
        };
    }
}

pub fn args_validation(args: &[String]) -> bool {
    if args.len() > 1 && args[1].starts_with('-') && args[1] != "-h" && args[1] != "--help" {
        println!("Opción no válida.");
        return false;
    }

    true
}

pub fn args_management(args: &[String]) -> bool {
    if args.len() == 2 && (args[1] == "-h" || args[1] == "--help") {
        show_help();
        return true;
    } else if args.len() == 3 {
        let username = &args[1];
        let password = &args[2];

        let db = match db::Database::initialize("app.db") {
            Ok(db) => db,
            Err(e) => {
                println!("{}", e);
                return true;
            }
        };

        let conn = db.get_connection();
        let user_manager = usuario::UserManager::new(conn);

        match user_manager.comprobar_usuario(username, password) {
            Ok(Some(usuario)) => println!("Usuario autenticado. ID: {}", usuario.id.unwrap()),
            Ok(None) => println!("Usuario o contraseña incorrectos."),
            Err(e) => println!("Error al verificar el usuario: {}", e),
        }
        return true;
    } else if args.len() > 3 {
        println!("Demasiados argumentos");
        show_help();
        return true;
    }

    false
}

pub fn autenticar_usuario(user_manager: &UserManager) {
    print!("Introduce el nombre de usuario: ");
    io::stdout().flush().unwrap();
    let mut username: String = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line.");

    print!("Introduce la contraseña: ");
    io::stdout().flush().unwrap();
    let mut password: String = String::new();
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line.");

    match user_manager.comprobar_usuario(username.trim(), password.trim()) {
        Ok(Some(usuario)) => println!("Usuario autenticado. ID: {}", usuario.id.unwrap()),
        Ok(None) => println!("Usuario o contraseña incorrectos."),
        Err(e) => println!("Error al verificar el usuario: {}", e),
    }
}

pub fn agregar_usuario(user_manager: &UserManager) {
    print!("Introduce el nombre de usuario: ");
    io::stdout().flush().unwrap();
    let mut username: String = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line.");

    print!("Introduce la contraseña: ");
    io::stdout().flush().unwrap();
    let mut password: String = String::new();
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line.");

    let usuario = Usuario {
      id: None,
      username: username.trim().to_string(),
      password: password.trim().to_string(),
    };

    match user_manager.agregar_usuario(&usuario) {
      Ok(_) => println!("Usuario añadido correctamente."),
      Err(e) => println!("Error al añadir usuario: {}", e),
    }
}
