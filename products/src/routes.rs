use actix_web::web;
use crate::handlers::{create_product, delete_product, get_product, get_products, update_product};
pub struct ProductsRoutes;

impl ProductsRoutes {
    pub fn get_routes(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::scope("/products")
                .service(
                    web::resource("")
                        .route(web::get().to(get_products))
                )
                .service(
                    web::resource("/")
                        .route(web::get().to(get_products))
                )
                .service(
                    web::resource("/new")
                        .route(web::post().to(create_product))
                )
                .service(
                    web::resource("/{id}")
                        .route(web::get().to(get_product))
                        .route(web::delete().to(delete_product))
                        .route(web::put().to(update_product))
                )


        );
    }
}
