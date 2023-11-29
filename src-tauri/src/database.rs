use mysql::{prelude::Queryable, *};
use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    username: String,
    email: String,
}

pub fn get_users(mut conn: PooledConn) -> std::result::Result<Vec<User>, mysql::Error> {
    let users = conn.query_map("SELECT * FROM users", |(id, username, email)| {
        let _id: u32 = id;
        let username: String = username;
        let email: String = email;

        User { username, email }
    });

    return users;
}

pub fn store_user(
    mut conn: mysql::PooledConn,
    username: String,
    email: String,
    password: String,
) -> bool {
    let params: (&str, &str, &str) = (&username, &email, &password);

    match conn.exec_batch(
        r"INSERT INTO users (username, email, password) VALUES (?, ?, ?)",
        std::iter::once(params),
    ) {
        Ok(_) => {
            println!("User data stored successfully!");
            true
        }
        Err(err) => {
            eprintln!("Error storing user data: {}", err);
            false
        }
    }
}

pub fn login(mut conn: mysql::PooledConn, username: String, password: String) -> bool {
    // Query the database to check if the given username and password match
    let query = format!(
        "SELECT COUNT(*) FROM users WHERE username = '{}' AND password = '{}'",
        username, password
    );

    let user_query = conn.query_first::<i32, &str>(&query);

    match user_query {
        Ok(Some(count)) => {
            // If count is 1, the username and password combination exists
            if count == 1 {
                println!("Login successful!");
                true
            } else {
                println!("Incorrect username or password.");
                false
            }
        }
        Ok(None) => {
            println!("User not found.");
            false
        }
        Err(err) => {
            eprintln!("Error querying user data: {}", err);
            false
        }
    }
}

pub fn update_user(
    mut conn: mysql::PooledConn,
    username: String,
    email: String,
    old_username: String,
) -> bool {
    let params: (&str, &str, &str) = (&username, &email, &old_username);
    match conn.exec_drop(
        r"UPDATE users SET username = ?, email = ? WHERE username = ?",
        params,
    ) {
        Ok(_) => {
            println!("User data updated successfully!");
            true
        }
        Err(err) => {
            eprintln!("Error updating user data: {}", err);
            false
        }
    }
}

pub fn delete_user(mut conn: mysql::PooledConn, username: String) -> bool {
    let params: Vec<&str> = vec![&username];

    match conn.exec_drop(r"DELETE FROM users WHERE username = :username", params) {
        Ok(_) => {
            println!("User deleted successfully!");
            true
        }
        Err(err) => {
            eprintln!("Error storing user data: {}", err);
            false
        }
    }
}

pub fn connect() -> PooledConn {
    let url = "mysql://root:@localhost:3306/tauri_first_app";
    let pool = Pool::new(url).unwrap();
    let conn = pool.get_conn().unwrap();
    return conn;
}
