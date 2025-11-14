// c2-server/src/main.rs
use actix_web::{web, App, HttpServer, HttpResponse, Result};
use actix_files::{Files, NamedFile};
use std::path::PathBuf;

// API handlers
async fn get_agents() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(vec![
        serde_json::json!({"id": 1, "hostname": "server-01", "status": "online"}),
    ]))
}

async fn get_tasks() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(vec![
        serde_json::json!({"id": 1, "command": "backup", "status": "completed"}),
    ]))
}

// Serve index.html per tutte le route non-API (per React Router)
async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("./frontend/dist/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // API routes
            .service(
                web::scope("/api")
                    .route("/agents", web::get().to(get_agents))
                    .route("/tasks", web::get().to(get_tasks))
            )
            // Serve static files (JS, CSS, images)
            .service(
                Files::new("/assets", "./frontend/dist/assets")
                    .use_last_modified(true)
            )
            // Fallback a index.html per SPA routing
            .default_service(web::get().to(index))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
