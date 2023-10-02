use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;
use sqlx::PgPool;
use env_logger::Env;

#[tokio::main]
async fn main() -> std::io::Result<()>{
    // init() call set_logger() internally RUST_LOG env overwrites the default filter
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    // Panic if we can't read the configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect Postgres.");
    // We have removed the hard-cored `8000` - it's now coming from our settings!
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}