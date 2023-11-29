// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use mysql::PooledConn;

mod database;

#[tauri::command]
fn get_users() -> Result<String, String> {
    let conn: PooledConn = database::connect();
    let users_result: Result<Vec<database::User>, mysql::Error> = database::get_users(conn);

    match users_result {
        Ok(users) => {
            // Convert the vector of users to a JSON string
            let users_json = serde_json::to_string(&users).map_err(|err| err.to_string())?;
            Ok(users_json)
        }
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
fn login(username: String, password: String) -> bool {
    let conn: PooledConn = database::connect();
    let user_result: bool = database::login(conn, username, password);
    if user_result {
        return true;
    } else {
        return false;
    }
}

#[tauri::command]
fn store_user(username: String, email: String, password: String) -> bool {
    // println!("Username is {}", username);
    let conn: PooledConn = database::connect();
    let user_result: bool = database::store_user(conn, username, email, password);
    if user_result {
        return true;
    } else {
        return false;
    }
}

#[tauri::command]
fn delete_user(username: String) -> bool {
    let conn: PooledConn = database::connect();
    let deleted_user: bool = database::delete_user(conn, username);
    if deleted_user {
        return true;
    } else {
        return false;
    }
}

#[tauri::command]
fn update_user(username: String, email: String, old_username: String) -> bool {
    let conn: PooledConn = database::connect();
    let updated_user: bool = database::update_user(conn, username, email, old_username);
    if updated_user {
        return true;
    } else {
        return false;
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_users,
            store_user,
            delete_user,
            update_user,
            login
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
