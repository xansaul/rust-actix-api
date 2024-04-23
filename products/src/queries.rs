use sea_orm::*;
use sea_orm::prelude::Uuid;
use models::product::{self ,Entity as ProductEntity, Model as ProductModel, ActiveModel as ProductActiveModel};
use serde::{Deserialize, Serialize};
use chrono::Utc;

use crate::handlers::ProductJsonRequest;

#[derive(Serialize, Deserialize)]


pub struct Query;

impl Query {

    pub async fn get_product_by_id(conn: &DatabaseConnection, id: i32)->Option<ProductModel>{
        let product_result = ProductEntity::find_by_id(id)
            .one(conn)
            .await.unwrap();
        
        product_result
    }


    pub async fn create_product(conn: &DatabaseConnection, product_data :ProductJsonRequest)-> Result<ProductActiveModel, DbErr>{
            
        product::ActiveModel {
            id: sea_orm::NotSet,
            name: sea_orm::Set(product_data.name.to_owned()),
            price: sea_orm::Set(product_data.price),
            uuid: sea_orm::Set(Uuid::new_v4()),
            created_at: sea_orm::Set(Utc::now().naive_utc()),
            ..Default::default()
        }.save(conn).await

    }

    pub async fn delete_product(conn: &DatabaseConnection, id: i32)->Option<Result<DeleteResult, DbErr>> {
        let product_to_delete = Query::get_product_by_id(conn, id).await;

        match product_to_delete {
            Some(product) => Some(product.delete(conn).await),
            None => None
        }
    }

    pub async fn find_products_in_page(
        db: &DbConn,
        page: u64,
        posts_per_page: u64,
    ) -> Result<(Vec<product::Model>, u64), DbErr> {
        
        let paginator = ProductEntity::find().paginate(db, posts_per_page);

        let num_pages = paginator.num_pages().await?;

        paginator.fetch_page(page - 1).await.map(|p| (p, num_pages))
    }

    pub async fn update_product(conn: &DatabaseConnection, id: i32, data: ProductJsonRequest, )->Option<ProductModel>{
        let product = Query::get_product_by_id(conn, id).await;

        match product {
            Some(product) => {           
                let mut product: ProductActiveModel = product.into();

                product.name = Set(data.name.to_owned());
                product.price = Set(data.price);

                Some(product.update(conn).await.unwrap())
            }
            None => None
        }
    }
}
