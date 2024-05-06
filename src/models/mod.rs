use diesel::prelude::*;
use diesel::PgConnection;
use std::env;

pub mod collection;
pub mod nft;
pub mod nft_trait;
pub mod schema;
pub mod user;

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
