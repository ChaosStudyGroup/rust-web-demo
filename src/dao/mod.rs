#[macro_export]
macro_rules! sql_query_one (
    ($sql: expr, $($bind: expr),*) => ({
        let pool = match db::mysql::get_pool() {
            Some(p) => p,
            None => return Err("mysql get pool failed".into()),
        };

        match sqlx::query_as(&$sql)$(.bind($bind))*.fetch_one(pool).await {
            Ok(u) => Ok(Some(u)),
            Err(e) => match e {
                sqlx::Error::RowNotFound => Ok(None),
                _ => Err(e.into())
            },
        }
    });
    ($sql: expr) => (query_one!($sql,));
);

#[macro_export]
macro_rules! sql_insert (
    ($sql: expr, $($bind: expr),*) => ({
        use tokio::stream::StreamExt;

        let mut conn = match db::mysql::get_pool() {
            Some(p) => p.acquire().await?,
            None => return Err("mysql get pool failed".into()),
        };

        sqlx::query(&$sql)$(.bind($bind))*.execute(&mut conn).await?;

        let sql = r#"select last_insert_id()"#;
        sqlx::query(sql).fetch(&mut conn).next().await?.unwrap().get::<u64>(0)
    });
    ($sql: expr) => (query_one!($sql,));
);

#[macro_export]
macro_rules! sql_update (
    ($sql: expr, $($bind: expr),*) => ({
        let pool = match db::mysql::get_pool() {
            Some(p) => p,
            None => return Err("mysql get pool failed".into()),
        };

        Ok(sqlx::query(&$sql)$(.bind($bind))*.execute(pool).await? >= 1)
    });
    ($sql: expr) => (query_one!($sql,));
);

#[macro_export]
macro_rules! sql_result (
    ($output: ty) => (Result<$output, Box<dyn std::error::Error>>);
);

pub mod rbac;
