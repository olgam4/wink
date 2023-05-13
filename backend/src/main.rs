#[macro_use]
extern crate rocket;

use rand::{distributions::Alphanumeric, Rng};
use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use rocket::{fairing, Build, Rocket};
use rocket_db_pools::sqlx;
use rocket_db_pools::{Connection, Database};
use serde::{Serialize, Deserialize};
use sqlx::Row;

#[derive(Database)]
#[database("db_sqlite")]
struct Db(sqlx::SqlitePool);

fn generate_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WinkCreate {
    url: String,
}

#[post("/wink", data = "<wink>")]
async fn create_wink(mut db: Connection<Db>, wink: Json<WinkCreate>) -> Option<String> {
    let name = generate_string(8);

    let result = sqlx::query("INSERT INTO links (name, url) VALUES (?, ?)")
        .bind(&name)
        .bind(&wink.url)
        .execute(&mut *db)
        .await;

    match result {
        Ok(_) => {
            println!("Success");
            Some(name)
        }
        Err(e) => {
            println!("Error: {}", e);
            None
        }
    }
}

#[get("/wink/<name>")]
async fn get_wink(name: String, mut db: Connection<Db>) -> Option<String> {
    let result = sqlx::query("SELECT url FROM links WHERE name = ?")
        .bind(&name)
        .fetch_one(&mut *db)
        .await;

    match result {
        Ok(row) => {
            println!("Success");
            Some(row.get(0))
        }
        Err(e) => {
            println!("Error: {}", e);
            None
        }
    }
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match Db::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("db/migrations").run(&db.0).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("SQLx Stage", |rocket| async {
        rocket
            .attach(Db::init())
            .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
            .mount("/", routes![create_wink, get_wink])
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(stage())
}
