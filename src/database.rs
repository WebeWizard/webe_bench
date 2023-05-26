use std::env;
use std::time::SystemTime;

use mysql_async::prelude::*;
use mysql_async::{OptsBuilder, Pool};

pub struct BenchResult {
    id: u64,
    run_id: u64,
    url: String,
    total_count: usize,
    concurrency: usize,
    success_count: usize,
    error_count: usize,
    start_time: std::time::SystemTime,
    finish_time: std::time::SystemTime,
    elapsed_time_ms: usize,
    req_per_sec: f64,
}

impl BenchResult {
    pub fn new(
        id: u64,
        run_id: u64,
        url: String,
        total_count: usize,
        concurrency: usize,
        success_count: usize,
        error_count: usize,
        start_time: std::time::SystemTime,
        finish_time: std::time::SystemTime,
        req_per_sec: f64,
    ) -> Self {
        BenchResult {
            id,
            run_id,
            url,
            total_count,
            concurrency,
            success_count,
            error_count,
            start_time,
            finish_time,
            elapsed_time_ms: finish_time.duration_since(start_time).expect("Failed processing system time").as_millis() as usize,
            req_per_sec,
        }
    }
}

pub async fn upload_results(result: BenchResult) -> Result<(), ()> {
    let environment = env::var("ENVIRONMENT").unwrap_or("dev".to_owned());
    // load env variables for database
    let env_file_name = ".env.".to_owned() + &environment;
    dotenvy::from_filename(env_file_name).expect("Could not load appropriate .env file");

    let mysql_host = env::var("MYSQL_HOST").expect("Missing MYSQL_HOST environment variable");
    let mysql_port = env::var("MYSQL_PORT").expect("Missing MYSQL_PORT environment variable");
    let mysql_db = env::var("MYSQL_DATABASE").expect("Missing MYSQL_DATABASE environment variable");
    let mysql_user = env::var("MYSQL_USER").expect("Missing MYSQL_USER environment variable");
    let mysql_pass =
        env::var("MYSQL_PASSWORD").expect("Missing MYSQL_PASSWORD environment variable");

    // upload results
    // -- connect to mysql
    let connection_opts: OptsBuilder = OptsBuilder::default()
        .ip_or_hostname(mysql_host)
        .tcp_port(
            mysql_port
                .parse()
                .expect("Could not parse MYSQL_PORT to int"),
        )
        .db_name(Some(mysql_db))
        .user(Some(mysql_user))
        .pass(Some(mysql_pass));
    let pool = Pool::new(connection_opts);
    let conn = pool.get_conn().await.expect("Could not connect to mysql");

    let start_time_secs = result.start_time.duration_since(SystemTime::UNIX_EPOCH).expect("Error processing system time").as_secs_f32();
    let finish_time_secs = result.finish_time.duration_since(SystemTime::UNIX_EPOCH).expect("Error processing system time").as_secs_f32();

    let sql = format!(
        r"INSERT INTO results
      (id, run_id, url, total_count, concurrency, attempt_count, success_count, error_count, start_timestamp, finish_timestamp, elapsed_time_ms, req_per_sec)
      VALUES
      ('{:x}','{:x}','{}',{}, {}, {}, {}, {}, FROM_UNIXTIME({}), FROM_UNIXTIME({}), {}, {})",
        result.id, result.run_id, result.url, result.total_count, result.concurrency, (result.success_count + result.error_count), result.success_count, result.error_count, start_time_secs, finish_time_secs, result.elapsed_time_ms, result.req_per_sec
    );

    println!("{:?}", sql);

    sql.ignore(conn)
        .await
        .expect("Failed to upload results to mysql");
    return Ok(());
}
