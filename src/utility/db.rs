pub mod mysql {
    use std::time::Duration;

    use once_cell::sync::OnceCell;
    use sqlx::{MySqlConnection, MySqlPool, pool::PoolConnection, Transaction};

    static MYSQL_POOL: OnceCell<MySqlPool> = OnceCell::new();

    type MySqlTransaction = Transaction<PoolConnection<MySqlConnection>>;

    pub async fn create_pool() -> MySqlPool {
        MySqlPool::builder()
            .min_size(crate::conf::global().mysql.min)
            .max_size(crate::conf::global().mysql.max)
            .connect_timeout(Duration::from_secs(10))
            .max_lifetime(Duration::from_secs(1800))
            .idle_timeout(Duration::from_secs(600))
            .build(&crate::conf::global().mysql.dsn)
            .await.expect("mysql server disconnect")
    }

    pub fn init_pool(pool: MySqlPool) {
        assert!(MYSQL_POOL.set(pool).is_ok());
    }

    pub fn get_pool() -> Option<&'static MySqlPool> {
        MYSQL_POOL.get()
    }

    pub async fn get_connection() -> Option<PoolConnection<MySqlConnection>> {
        let pool = MYSQL_POOL.get();

        if pool.is_none() {
            return None;
        }

        pool.unwrap().acquire().await.ok()
    }

    pub async fn get_transaction() -> Option<MySqlTransaction> {
        let pool = MYSQL_POOL.get();

        if pool.is_none() {
            return None;
        }

        pool.unwrap().begin().await.ok()
    }
}

pub mod redis {
    use std::time::Duration;

    use mobc_redis::{redis, RedisConnectionManager};
    use once_cell::sync::OnceCell;

    static REDIS_POOL: OnceCell<RedisPool> = OnceCell::new();

    type RedisPool = mobc::Pool<RedisConnectionManager>;
    type RedisConn = mobc::Connection<RedisConnectionManager>;

    pub fn create_pool() -> RedisPool {
        let manager = RedisConnectionManager::new({
            let addr = crate::conf::global().redis_addr();
            redis::Client::open(addr.as_str()).expect("redis server disconnect")
        });

        mobc::Pool::builder()
            .max_idle(crate::conf::global().redis.min)
            .max_open(crate::conf::global().redis.max)
            .build(manager)
    }

    pub fn init_pool(pool: RedisPool) {
        assert!(REDIS_POOL.set(pool).is_ok());
    }

    pub fn get_pool() -> Option<&'static RedisPool> {
        REDIS_POOL.get()
    }

    pub async fn get_connection() -> Option<RedisConn> {
        let pool = REDIS_POOL.get();

        if pool.is_none() {
            return None;
        }

        pool.unwrap().get_timeout(Duration::from_secs(5)).await.ok()
    }
}
