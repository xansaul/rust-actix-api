use actix_web::{web, HttpRequest, HttpResponse, Responder};
use config::app_state::AppState;
use serde::{Deserialize, Serialize};

use crate::responses::{Error, ProductsResponse, };
use crate::queries::Query;

#[derive(Debug, Deserialize)]
pub struct Params {
    page: Option<u64>,
    posts_per_page: Option<u64>,
}

pub async fn get_products(
    req: HttpRequest, 
    data: web::Data<AppState>) -> impl Responder {

    let params = web::Query::<Params>::from_query(
        req.query_string()
    ).unwrap();

    let page = params.page.unwrap_or(1);
    let products_per_page = params.posts_per_page.unwrap_or(5);

    let conn = &data.conn;
    
    let (products, pages) = Query::find_products_in_page(
        conn, 
        page, 
        products_per_page
    ).await.expect("Cannot find posts in page");

    HttpResponse::Ok().json(ProductsResponse { pages, products })
}


#[derive(Serialize, Deserialize)]
pub struct ProductJsonRequest {
    pub name: String,
    pub price: f32
}

pub async fn create_product(
    body: web::Json<ProductJsonRequest>, 
    app_data: web::Data<AppState>) -> impl Responder {

    let conn = &app_data.conn;
    let new_product = Query::create_product(conn, body.into_inner()).await;
    
    match new_product {
        Ok(new_product) => {

            let product = Query::get_product_by_id(
                conn, new_product.id.unwrap()
            ).await.unwrap();
            HttpResponse::Created().json(product)

        }
        Err(error) => HttpResponse::InternalServerError()
        .json(Error { error: error.to_string() }), 
    }
}


pub async fn get_product(
    path: web::Path<i32>, 
    app_data: web::Data<AppState>)-> impl Responder {

    let id = path.into_inner();
    let conn = &app_data.conn;
    let new_product = Query::get_product_by_id(conn, id).await;

    match new_product {
        Some(new_product) => HttpResponse::Found().json(new_product),
        None => HttpResponse::NotFound()
        .json(Error { 
            error: format!("Product with id: {id} not found")
         })
    }

}

pub async fn delete_product(
    path: web::Path<i32>, 
    app_data: web::Data<AppState>)-> impl Responder {

    let conn = &app_data.conn;
    let id = path.into_inner();

    let product_to_delete = Query::delete_product(conn, id).await;

    if let Some(product_to_delete)  = product_to_delete {

        match product_to_delete {
            Ok(_) => HttpResponse::NoContent().finish(),
            Err(error) => HttpResponse::InternalServerError()
            .json(Error { error: error.to_string() })
        }

    }else {
        HttpResponse::NotFound().json(Error { 
            error: format!("Product with id: {id} not found")
         })
    }
}


pub async fn update_product(
    path: web::Path<i32>,
    body: web::Json<ProductJsonRequest>,
    app_data: web::Data<AppState>)-> impl Responder{

    
    let conn = &app_data.conn;
    let id = path.into_inner();
    
    let product_update_result = Query::update_product(conn, id, body.into_inner()).await;

    if let Some(product_updated)  = product_update_result {

        HttpResponse::Ok().json(product_updated)

    }else {
        HttpResponse::NotFound().json(Error { 
            error: format!("Product with id: {id} not found")
        })
    }

}



