use sea_orm::{Database, DatabaseConnection};


pub struct DatabaseConnectionApp;

impl DatabaseConnectionApp {
    pub async fn get_database() -> DatabaseConnection {
        
        let conn_str = (*crate::envs::DATABASE_URL).clone();
        
        match Database::connect(conn_str).await {
            Ok(db) => db,
            Err(error) => panic!("Connection  error: {}", error),
        }
    }
}