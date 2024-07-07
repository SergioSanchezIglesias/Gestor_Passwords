use rusqlite::{params, Connection, Result};

pub struct UserManager<'a> {
    conn: &'a Connection,
}

pub struct Usuario {
    pub id: Option<i32>,
    pub username: String,
    pub password: String,
}

pub struct PasswordUsuario {
    pub user_id: i32,
    pub servicio: String,
    pub username: String,
    pub password: String,
}

impl<'a> UserManager<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        UserManager { conn }
    }

    // Agregar usuario a la base de datos
    pub fn agregar_usuario(&self, usuario: &Usuario) -> Result<()> {
        self.conn.execute(
            "INSERT INTO Usuarios (Username, Password) VALUES (?1, ?2)",
            params![usuario.username, usuario.password],
        )?;
        Ok(())
    }

    // Eliminar usuario de la base de datos
    // pub fn eliminar_usuario(&self, username: &str) -> Result<()> {
    //     self.conn.execute(
    //         "DELETE FROM Usuarios WHERE Username = ?1",
    //         params![username],
    //     )?;
    //     Ok(())
    // }

    // Obtener un usuario
    pub fn comprobar_usuario(&self, username: &str) -> Result<Option<Usuario>> {
        let mut stmt = self
            .conn
            .prepare("SELECT UserID, Username, Password FROM Usuarios WHERE Username = ?1")?;

        let mut usuario_iter = stmt.query_map(params![username], |row| {
            Ok(Usuario {
                id: Some(row.get(0)?),
                username: row.get(1)?,
                password: row.get(2)?,
            })
        })?;

        if let Some(usuario) = usuario_iter.next() {
            return Ok(Some(usuario?));
        }
        Ok(None)
    }

    // Listar todos los usuarios
    // pub fn listar_usuarios(&self) -> Result<Vec<Usuario>> {
    //     let mut smt = self
    //         .conn
    //         .prepare("SELECT UserID, Username, Password FROM Usuarios")?;
    //     let usuario_iter = smt.query_map([], |row| {
    //         Ok(Usuario {
    //             id: Some(row.get(0)?),
    //             username: row.get(1)?,
    //             password: row.get(2)?,
    //         })
    //     })?;

    //     let mut usuarios = Vec::new();
    //     for usuario in usuario_iter {
    //         usuarios.push(usuario?);
    //     }
    //     Ok(usuarios)
    // }

    pub fn agregar_password(&self, password_usuario: &PasswordUsuario) -> Result<()> {
        self.conn.execute(
            "INSERT INTO Passwords (UserID, Nombre_del_Servicio, Username, Password) VALUES (?1, ?2, ?3, ?4)",
            params![password_usuario.user_id, password_usuario.servicio, password_usuario.username, password_usuario.password],
        )?;

        Ok(())
    }

    pub fn obtener_password(
        &self,
        user_id: i32,
        servicio: &String,
    ) -> Result<Option<(String, String)>> {
        let mut stmt = self.conn.prepare("SELECT Username, Password FROM Passwords WHERE UserID = ?1 AND Nombre_del_Servicio = ?2")?;
        let mut password_iter = stmt.query_map(params![user_id, servicio], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })?;

        if let Some(password) = password_iter.next() {
            return Ok(Some(password?));
        }

        Ok(None)
    }

    pub fn obteneer_all_passwords(&self, user_id: i32) -> Result<Vec<(String, String, String)>> {
        let mut stmt = self.conn.prepare(
            "SELECT Username, Password, Nombre_del_Servicio FROM Passwords WHERE UserID = ?1",
        )?;
        let passwords_iter = stmt.query_map(params![user_id], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })?;

        let mut passwords = Vec::new();
        for password in passwords_iter {
            passwords.push(password?);
        }

        Ok(passwords)
    }
}
