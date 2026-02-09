use core::panic;

use tokio_postgres::{Client, NoTls};

async fn db_connection() -> Client {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=mortwain dbname=rustDB",
     NoTls).await.unwrap();

    tokio::spawn(async move{
        if let Err(e) = connection.await{
            eprintln!("Something went wrong! : {}", e);
        }
    });

    client
}

struct User{
    id:i32,
    first_name:String,
    last_name:String,
}

async fn save_user(client:&Client, user:&User){
    let result = client.query(
        "INSERT INTO app.Users(id, first_name, last_name) VALUES($1, $2, $3);",
    &[&user.id, &user.first_name, &user.last_name] ).await;
    
    match result {
        Ok(_) => {
            println!("User Saved!");
        },
        Err(e) => {
            panic!("Save Error - {e}");
        }
    }
}

async fn update_user(client:&Client, user:&User){
    let result= client.execute(
        "UPDATE app.Users SET first_name=$1, last_name=$2 WHERE id=$3;",
        &[&user.first_name, &user.last_name, &user.id] ).await;

    match result {
        Ok(v) => {
            println!("{} lines affected!", v);
        },
        Err(e) => {
            panic!("Update Error - {e}");
        }
    }    
}

async fn delete_user(client:&Client, id:i32){
    let result = client.execute(
        "DELETE FROM app.Users WHERE id=$1",
        &[&id]).await;

    match result {
        Ok(v) => {
            println!("{} lines affected.", v);
        },
        Err(e) => {
            panic!("Delete Error - {e}");
        }
    }
}

#[tokio::main]
async fn main() {
    let client:Client = db_connection().await;

    // let user:User = User{id:6, first_name:"CCC".to_string(), last_name:"ccc".to_string()};
    // save_user(&client, &user).await;

    // let new_user:User = User{id:6, first_name:"BBB".to_string(), last_name:"bbb".to_string()};
    // update_user(&client, &new_user).await;

    // delete_user(&client, 6).await;
}