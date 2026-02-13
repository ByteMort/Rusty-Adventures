use sqlx::{Error, PgPool, postgres::PgPoolOptions};

async fn db_connection() -> PgPool{
    PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://mortwain@localhost/rustDB")
        .await
        .expect("Something went wrong with Db Connection")
}

#[derive(Debug)]
#[derive(Clone)]
struct User{
    id: i32,
    first_name: Option<String>,
    last_name: Option<String>,
}

impl User{
    fn get_info(&self){
        println!("Id: {}\tFirst Name: {}\t\tLast Name: {}", self.id, self.first_name.as_ref().unwrap(), self.last_name.as_ref().unwrap());
    }
}

async fn insert_user(pool:&PgPool, user:&User) -> Result<(), Error> {
    // Use bind() just with query
    // query! takes params

    sqlx::query!("INSERT INTO app.Users(id, first_name, last_name) VALUES($1, $2, $3)"
        ,user.id, user.first_name, user.last_name)
        .execute(pool).await?;

    Ok(())
}

async fn update_user(pool:&PgPool, user:&User) -> Result<(), Error>{
    sqlx::query!("UPDATE app.Users SET first_name=$1, last_name=$2 WHERE id=$3"
        ,user.first_name, user.last_name, user.id)
        .execute(pool)
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() {

    let pool = db_connection().await;

    let new_user:User = User{id:999, first_name:Some("Test".to_string()), last_name:Some("Test".to_string())};
    let _ = insert_user(&pool, &new_user).await;

    let rows:Vec<User> = sqlx::query_as!(User,
        "SELECT id, first_name, last_name FROM app.Users")
        .fetch_all(&pool)
        .await.unwrap();

    let mut user:User = rows.first().unwrap().clone();
    user.first_name = Some("ALIIIIII".to_string());
    let _ = update_user(&pool, &user).await;

    for row in rows{
        row.get_info();
    }
}
