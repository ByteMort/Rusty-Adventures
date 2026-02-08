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
    client.query(
        "INSERT INTO app.Users(id, first_name, last_name) VALUES($1, $2, $3);",
    &[&user.id, &user.first_name, &user.last_name] ).await.unwrap();
}

async fn update_user(client:&Client, user:&User){
    let affected:u64= client.execute(
        "UPDATE app.Users SET first_name=$1, last_name=$2 WHERE id=$3;",
        &[&user.first_name, &user.last_name, &user.id] ).await.unwrap();

    println!("{} lines affected.", affected);
}

async fn delete_user(client:&Client, id:i32){
    let affected = client.execute(
        "DELETE FROM app.Users WHERE id=$1",
        &[&id]).await.unwrap();
    println!("{} lines affected.", affected);
}

#[tokio::main]
async fn main() {
    let client:Client = db_connection().await;

    // let user:User = User{id:5, first_name:"Byte".to_string(), last_name:"Mort".to_string()};
    // save_user(&client, &user).await;

    // let new_user:User = User{id:5, first_name:"Ben".to_string(), last_name:"Ten".to_string()};
    // update_user(&client, &new_user).await;

    // delete_user(&client, 5).await;
}