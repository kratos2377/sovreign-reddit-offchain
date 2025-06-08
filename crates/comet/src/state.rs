use deadpool_lapin::{Config as RabbitConfig, Manager as RabbitManager, Pool as RabbitPool};
use deadpool_redis::{Config as RedisConfig, Pool as RedisPool};



#[derive(Clone)]
pub struct AppDBState {
   pub redis_pool: RedisPool,
   pub rabbit_pool: RabbitPool,
}