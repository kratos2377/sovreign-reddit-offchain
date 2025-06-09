use redis::aio::MultiplexedConnection;
use sea_orm::DatabaseConnection;




#[derive(Clone)]

pub struct DBState {
    pub connection: DatabaseConnection,
    pub redis_connection: MultiplexedConnection
}