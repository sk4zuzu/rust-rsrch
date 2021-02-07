#[tokio::main]
async fn main() -> serve_static::Result<()> {
    pretty_env_logger::init();
    serve_static::ServeStatic::new("http://0.0.0.0:1234".to_string()).serve().await
}
