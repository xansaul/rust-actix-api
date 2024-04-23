use actix_web::{web, App, HttpServer};
use products::routes::ProductsRoutes;
use config::database::DatabaseConnectionApp;
use config::envs::{HOST, PORT};
use config::app_state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let conn = DatabaseConnectionApp::get_database().await;
    
    let app_state = AppState{ conn };

    HttpServer::new(move || {
     App::new()
     .app_data(web::Data::new(app_state.clone()))
     .configure(ProductsRoutes::get_routes)
    })
    .bind((HOST.clone(), *PORT))?
    .run()
    .await
}

