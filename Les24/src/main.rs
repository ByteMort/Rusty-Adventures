use core::panic;

use sqlx::{Error, PgPool, postgres::{PgPoolOptions, PgQueryResult, PgRow}, query_as};

async fn db_connection() -> PgPool{
    PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://mortwain@localhost/rustDB")
        .await
        .expect("Something went wrong with db Connection")
}

async fn get_users(pool:&PgPool) -> Vec<User> {
    let result:Result<Vec<User>, Error> = sqlx::query_as!(User, "SELECT * FROM App.Users")
    .fetch_all(pool)
    .await;

    let users:Vec<User> = match result{
        Ok(v) => {
            v
        },
        Err(e) => {
            panic!("Something went wrong with get_users : {}", e);
        },
    };
    
    users
}

async fn add_user(pool:&PgPool, user:&User) -> User{
    let added: Result<User, Error> = sqlx::query_as!(User, 
        "INSERT INTO App.Users(id, first_name, last_name) VALUES ($1, $2, $3) RETURNING id, first_name, last_name",
         user.id, user.first_name, user.last_name)
        .fetch_one(pool)
        .await;

    let user = match added{
        Ok(v) => {
            v
        },
        Err(e) => {
            panic!("Something went wrong while adding : {}", e)
        },
    };

    user
}

async fn update_user(pool:&PgPool, user:&User) -> User{
    let updated:Result<Option<User>, Error> = sqlx::query_as!(User, "UPDATE App.Users SET first_name=$1, last_name=$2 WHERE id=$3 RETURNING id, first_name, last_name",
     user.first_name, user.last_name, user.id)
     .fetch_optional(pool).await;

    /* fetch_one()
    let user:User = match updated{
        Ok(v) => {
            v
        },
        Err(e) => {
            panic!("Something went wrong while updating : {}", e)
        },
    };*/
    
    let user:User = match updated{
        Ok(Some(v)) => {
            v
        },
        Ok(None) => {
            panic!("No user found with id {}", user.id);
        }
        Err(e) => {
            panic!("Something went wrong while updating : {}", e);
        },
    };

    user
}

async fn delete_user(pool:&PgPool, id:i32){
    let delete:Result<PgQueryResult, Error> = sqlx::query!("DELETE FROM App.Users WHERE id=$1", id).execute(pool).await;

    match delete {
        Ok(v) => {
            println!("Affected rows: {}", v.rows_affected());
        },
        Err(e) => {
            panic!("Something went wrong while deleting : {}", e);
        },
    };
}

struct User{
    id: i32,
    first_name: Option<String>,
    last_name: Option<String>
}

impl User{
    fn get_info(&self){
        println!("Id: {}\nFirst Name: {}\nLast Name: {}\n", self.id, self.first_name.as_ref().unwrap(), self.last_name.as_ref().unwrap());
    }
}

#[tokio::main]
async fn main() {
    let pool = db_connection().await;
    
    let users:Vec<User> = get_users(&pool).await;
    users.iter().for_each(|user|{
        user.get_info();
    });

    /*
    let new_user:User = User { id: 5, first_name: Some("Test User".to_string()), last_name: Some("Test User".to_string()) };
    let added_user:User = add_user(&pool, &new_user).await;
    println!("Added user is: ");
    added_user.get_info();
    */

    /*
    let new_user2:User = User{ id: 5, first_name: Some("My User".to_string()), last_name: Some("My User L".to_string()) };
    let updated_user:User = update_user(&pool, &new_user2).await;
    println!("Updated user is: ");
    updated_user.get_info();
    */

    // delete_user(&pool, 5).await;
}