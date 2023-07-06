use tide::Request;

#[derive(serde::Deserialize)]
struct Rust {
    name: String,
    version: f32,
}

async fn create(mut req: Request<()>) -> tide::Result<String> {
    let rust: Rust = req.body_json().await?;
    Ok(format!("Hello {}! Your Rust version is {}.", rust.name, rust.version))
}

#[tokio::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/rust").post(create);
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}
