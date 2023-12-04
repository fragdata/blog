use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{payload::PlainText, OpenApi, OpenApiService};
use color_eyre::{Result};

struct Api;

#[OpenApi]
impl Api {
    /// Hello world
    #[oai(path = "/", method = "get")]
    async fn index(&self) -> PlainText<&'static str> {
        PlainText("Hello World")
    }

    #[oai(path = "/blog", method = "get")]
    async fn blogs(&self) -> PlainText<&'static str> {
        PlainText("Blog posts are (antechronologically) : ")
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server("http://localhost:3000");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/", api_service).nest("/docs", ui);

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await;

    Ok(())
}