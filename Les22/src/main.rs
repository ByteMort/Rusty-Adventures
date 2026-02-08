use tokio_postgres::{Client, NoTls};

async fn setup_db() -> Client {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=mortwain dbname=rustDB", NoTls)
        .await
        .unwrap();
    
    tokio::spawn(async move {
        if let Err(e) = connection.await{
            eprintln!("Error - {}", e);
        }
    });
    
    client
}

async fn get_schemas(client: &Client){
    let rows = client
        .query(
            "SELECT schema_name
            FROM information_schema.schemata
            ORDER BY schema_name",
            &[]).await.unwrap();
    println!("Schemas:");
    for row in rows{
        let s:String = row.get(0);
        println!(" - {}", s);
    }
}

struct User{
    id:i32,
    first_name:String,
    last_name:String, 
}
impl User{
    fn get_user_info(&self){
        println!("User Id: {}", self.id);
        println!("User First Name: {}", self.first_name);
        println!("User Last Name: {}", self.last_name);
    }
}

async fn get_users(client: &Client) -> Vec<User> {
    let rows = client.query(
        "SELECT *
        FROM app.Users;", &[]).await.unwrap();
    
    let mut users:Vec<User> = Vec::new();
    for row in rows{
        let id:i32 = row.get("id");
        let first_name:String = row.get("first_name");
        let last_name:String = row.get("last_name");
        let user:User = User { id, first_name, last_name };
        users.push(user);
    }
    users
}

#[tokio::main]
async fn main() {
    
    let client:Client = setup_db().await;
    // println!("Client : {:?}", client);
    get_schemas(&client).await;

    println!();
    
    let users:Vec<User> = get_users(&client).await;
    users.iter().for_each(|u|{
        u.get_user_info();
        println!();
    });
}
