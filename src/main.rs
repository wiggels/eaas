use actix_files::Files;
use actix_web::{App, HttpServer, web};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api").configure(api::config))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", api::ApiDoc::openapi()),
            )
            .service(
                Files::new("/", "./static") // serve static dir
                    .index_file("index.html")
                    .default_handler(web::to(|| async {
                        actix_files::NamedFile::open("./static/index.html").unwrap()
                    })),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
