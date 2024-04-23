use std::env;
use dotenv::dotenv;

use lazy_static::lazy_static;

lazy_static!{
    pub static ref DATABASE_URL: String = set_database();
    pub static ref HOST: String = set_host();
    pub static ref PORT:  u16  = set_port();
}


fn set_database() -> String{
    dotenv().ok();
    env::var("DATABASE_URL").unwrap()
}

fn set_host() -> String{
    dotenv().ok();
    env::var("HOST").unwrap()
}

fn set_port() -> u16 {

    let puerto_str = env::var("PORT").expect("La variable de entorno PORT no está definida");

    let puerto: u16 = puerto_str
        .parse()
        .expect("No se pudo analizar el puerto como u16");

    puerto
}