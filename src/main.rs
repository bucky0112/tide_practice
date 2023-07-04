use tide::Request;

#[derive(serde::Deserialize)]
#[serde(default)]
struct NameParams {
    pub name: String,
}

impl Default for NameParams {
    fn default() -> Self {
        Self {
            name: "world".to_string(),
        }
    }
}

async fn handle_name(req: Request<()>) -> tide::Result<String> {
    let name = req
        .url()
        .query_pairs()
        .find(|(key, _)| key == "name")
        .map(|(_, value)| value);

    Ok(format!("Hello, {}!", name.unwrap_or("world".into())))
}
#[tokio::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/").get(handle_name);
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}
