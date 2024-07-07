use actix_files::NamedFile;
use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse, Result, middleware::Logger, http::header};
use env_logger::Env;
use std::path::PathBuf;
use log::info;

async fn serve_file(req: HttpRequest) -> Result<HttpResponse> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    let file_path = PathBuf::from("./dist").join(path);

    if file_path.is_file() {
        info!("Serving file: {:?}", file_path);
        let is_js = file_path.extension().and_then(|ext| ext.to_str()) == Some("js");
        let is_css = file_path.extension().and_then(|ext| ext.to_str()) == Some("css");
        
        let named_file = NamedFile::open(&file_path)?;
        let mut response = named_file.into_response(&req);
        
        // Aggiungi header Cache-Control per file JavaScript
        if is_js || is_css{
            response.headers_mut().insert(
                header::CACHE_CONTROL,
                header::HeaderValue::from_static("public, max-age=3600"),
            );
        }
        
        Ok(response)
    } else {
        info!("File not found, serving index: {:?}", file_path);
        Ok(NamedFile::open("./dist/app/index.html")?.into_response(&req))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::new().default_filter_or("info"));
    info!("Welcome to Snappy minimal power server!");
    info!("Starting server...");
    info!("Listening on http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(web::resource("/{filename:.*}").route(web::get().to(serve_file)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}