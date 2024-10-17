use std::io;
use serde::{Deserialize, Serialize};
use tokio;
use sqlx::{PgPool};
mod subtraction;


struct User {
    email: String,
    password: String,
}

#[tokio::main]
async fn main()-> Result<(), sqlx::Error> {

    let database_url = "postgres://your_user:your_password@localhost/your_db";
    let pool = PgPool::connect(database_url).await?;

    let mut email = String::new();
    let mut password = String::new();

    println!("Enter email: ");
    io::stdin().read_line(&mut email).expect("Failed to read line");

    println!("Enter password : ");
    io::stdin().read_line(&mut password).expect("Failed to read line");

    
    let email = email.trim().to_string();
    let password = password.trim().to_string();
    
    sqlx::query("INSERT INTO users (email, password) VALUES($1, $2)")
        .bind(&email)
        .bind(&password)
        .execute(&pool)
        .await?;



    println!("User added successfully!");

    Ok(())
}
