use dotenvy::dotenv;
use std::env;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{ Pool, PooledConnection, ConnectionManager, PoolError };

// use crate::db_connection::PgPool;
use rocket_sync_db_pools::database;

#[database("postgres_db")]
pub struct DbConn(PgPool);

//Connects to Postgres and call init pool
pub fn establish_connection() -> PgPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    init_pool(&database_url).expect("Failed to create pool")
}


//Creates a default R2D2 Postgres DB Pool
fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}


//this functions returns a connection from the Pool
pub fn pg_pool_handler(pool: &PgPool) -> Result<PgPooledConnection, PoolError> {
    let _pool = pool.get().unwrap();
    Ok(_pool)
}