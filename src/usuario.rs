use rusqlite::{params, Connection, Result};

pub struct UserManager<'a> {
    conn: &'a Connection,
}

pub struct Usuario {
    pub id: Option<i32>,
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
    pub fn comprobar_usuario(&self, username: &str, password: &str) -> Result<Option<Usuario>> {
        let mut stmt = self.conn.prepare(
            "SELECT UserID, Username, Password FROM Usuarios WHERE Username = ?1 AND Password = ?2",
        )?;

        let mut usuario_iter = stmt.query_map(params![username, password], |row| {
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
}
