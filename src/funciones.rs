use bcrypt::{hash, verify, DEFAULT_COST};
use colored::Colorize;
use std::io::{self, Write};

use crate::db;
use crate::usuario::UserManager;
use crate::usuario::Usuario;
use crate::usuario::{self, PasswordUsuario};

pub fn show_menu() {
    println!("\n1. Agregar usuario");
    println!("2. Autenticar usuario");
    println!("3. Salir");
    println!();
}

pub fn show_help() {
    println!("Uso:");
    println!("cargo run -- -h --help     Mostrar ayuda.");
    println!("cargo run -- <username> <password>     Entra a la aplicación estando autenticado.");
    println!("cargo run     Entra a la aplicación sin autenticar.");
    println!();
}

pub fn get_option() -> u32 {
    loop {
        let mut choice: String = String::new();

        print!("{}", "\nSeleccione una opción: ".yellow());
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line.");

        match choice.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("{}", "Introduce un número válido.".red()),
        };
    }
}

pub fn args_validation(args: &[String]) -> bool {
    if args.len() > 1 && args[1].starts_with('-') && args[1] != "-h" && args[1] != "--help" {
        println!("{}", "Opción no válida.".red());
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

        match user_manager.comprobar_usuario(username.trim()) {
            Ok(Some(usuario)) => {
                if verify(password.trim(), &usuario.password).unwrap_or(false) {
                    println!("{}", "Usuario autenticado correctamente.".green());
                    let user_id = usuario.id.unwrap_or(0);
                    menu_password_management(&user_manager, user_id);
                } else {
                    println!("{}", "\nUsuario o contraseña incorrecto.".red());
                }
            }
            Ok(None) => println!("{}", "Usuario o contraseña incorrectos.".red()),
            Err(_) => println!("{}", "Error al verificar el usuario".red()),
        }
        return true;
    } else if args.len() > 1 {
        println!("{}", "\nNúmero de argumentos incorrecto.".red());
        show_help();
        return true;
    }

    false
}

pub fn autenticar_usuario(user_manager: &UserManager) -> Option<Usuario> {
    print!("\nIntroduce el nombre de usuario: ");
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

    match user_manager.comprobar_usuario(username.trim()) {
        Ok(Some(usuario)) => {
            if verify(password.trim(), &usuario.password).unwrap_or(false) {
                println!("{}", "Usuario autenticado correctamente.".green());
                return Some(usuario);
            } else {
                println!("{}", "Usuario o contraseña incorrecto.".red());
            }
        }
        Ok(None) => println!("{}", "Usuario o contraseña incorrectos.".red()),
        Err(_) => println!("{}", "Error al verificar el usuario.".red()),
    }

    None
}

pub fn agregar_usuario(user_manager: &UserManager) {
    print!("\nIntroduce el nombre de usuario: ");
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

    let hashed_password: String = match hash(password.trim(), DEFAULT_COST) {
        Ok(hp) => hp,
        Err(_) => {
            println!("{}", "Error al cifrar la contraseña.".red());
            return;
        }
    };

    let usuario: Usuario = Usuario {
        id: None,
        username: username.trim().to_string(),
        password: hashed_password,
    };

    match user_manager.agregar_usuario(&usuario) {
        Ok(_) => println!("{}", "Usuario añadido correctamente.".green()),
        Err(_) => println!("{}", "Error al añadir usuario.".red()),
    }
}

pub fn menu_password_management(user_manager: &UserManager, user_id: i32) {
    loop {
        println!("\n1. Crear contraseña para un servicio.");
        println!("2. Obtener contraseña para un servicio.");
        println!("3. Obtener todas las contraseñas.");
        println!("4. Salir");

        let choice = get_option();

        match choice {
            1 => crear_user_password(user_manager, user_id),
            2 => obtener_user_password(user_manager, user_id),
            3 => obtener_all_user_passwords(user_manager, user_id),
            4 => break,
            _ => println!("{}", "Opción no válida, intente de nuevo.".red()),
        }
    }
}

pub fn crear_user_password(user_manager: &UserManager, user_id: i32) {
    print!("\nIntroduce el nombre del servicio: ");
    io::stdout().flush().unwrap();
    let mut servicio: String = String::new();
    io::stdin()
        .read_line(&mut servicio)
        .expect("Failed to read line.");

    print!("Introduce el nombre de usuario o correo del servicio: ");
    io::stdout().flush().unwrap();
    let mut username: String = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line.");

    print!("Introduce la contraseña del servicio: ");
    io::stdout().flush().unwrap();
    let mut password: String = String::new();
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line.");

    // Debido a que no se puede revertir el hash, de momento se almacena en texto claro

    // let hashed_password = match hash(password.trim(), DEFAULT_COST) {
    //     Ok(hp) => hp,
    //     Err(e) => {
    //         println!("Error al cifrar la contraseña: {}", e);
    //         return;
    //     }
    // };

    let password_usuario: PasswordUsuario = PasswordUsuario {
        user_id: user_id,
        servicio: servicio,
        username: username,
        // password: hashed_password,
        password: password,
    };

    match user_manager.agregar_password(&password_usuario) {
        Ok(_) => println!(
            "{}",
            "Contraseña para el servicio añadida correctamente.".green()
        ),
        Err(_) => println!(
            "{}",
            "Error al agregar la contraseña para el servicio.".red()
        ),
    }
}

pub fn obtener_user_password(user_manager: &UserManager, user_id: i32) {
    print!("Introduce el nombre del servicio: ");
    io::stdout().flush().unwrap();
    let mut servicio: String = String::new();
    io::stdin()
        .read_line(&mut servicio)
        .expect("Failed to read line.");

    match user_manager.obtener_password(user_id, &servicio) {
        Ok(Some((username, password))) => {
            print!(
                "Servicio: {} - Username o Correo: {} - Contraseña: {}",
                servicio.trim().green(),
                username.trim().green(),
                password.trim().green(),
            );
        }
        Ok(None) => println!(
            "{}",
            "No se encontró ninguna contraseña para ese servicio.".red()
        ),
        Err(_) => println!(
            "{}",
            "Error al obtener la contraseña para el servicio.".red()
        ),
    }
}

pub fn obtener_all_user_passwords(user_manager: &UserManager, user_id: i32) {
    match user_manager.obteneer_all_passwords(user_id) {
        Ok(passwords) => {
            for (username, password, servicio) in passwords {
                print!(
                    "Username o Correo: {} - Contraseña: {} - Servicio: {}\n",
                    username.trim().green(),
                    password.trim().green(),
                    servicio.trim().green(),
                );
            }
        }
        Err(_) => println!("{}", "Error al obtener todas las contraseñas.".red()),
    }
}
