use actix_web::{
    //middleware,
    web, 
    //App, HttpRequest, 
    HttpResponse, 
    //HttpServer, Responder, 
    Error,
    Result,
    //http::Method,

};

//use serde_derive::{Serialize, Deserialize};
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

use postgres::{Connection, TlsMode};
//use diesel::insert_into;
//use dotenv::dotenv;
//use std::env;

use crate::Server;
use crate::html_routes;
use crate::model::NewParams;
use crate::model::Params;

//pub fn establish_connection() -> ConnectionManager<diesel::PgConnection> {
//    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//    ConnectionManager::<PgConnection>::new(database_url)
//}

//#[derive(Clone)]
//pub struct Server {
//    pool: Pool<ConnectionManager<PgConnection>>,
//}
//
//impl Server {
//    pub fn new() -> Self {
//        dotenv().ok();
//        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
//        let manager = ConnectionManager::<PgConnection>::new(database_url);
//        let pool = Pool::builder()
//            .build(manager)
//            .expect("Failed to create pool.");
//        Server { pool }
//    }
//}

pub fn add_new_user_system(params: web::Form<NewParams>, server: web::Data<Server>) -> Result<HttpResponse, Error> {
    let result;
    if params.new_pass == params.retype_new_pass {
        result = insert_database_new_user(params, server)
    }else{
        result = html_routes::add_new_user()
    };
    result
}

pub fn insert_database_new_user(params: web::Form<NewParams>, server: web::Data<Server>) -> Result<HttpResponse, Error>{
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let conn = Connection::connect(database_url, TlsMode::None).unwrap();
    conn.execute("INSERT INTO user_info (id, name, password) VALUES ($1, $2, $3)", &[&params.new_id, &params.new_name, &params.new_pass]);
    //use crate::schema::user_info::dsl;
    //let server = Server::new();
    //diesel::insert_into(dsl::user_info)
    //    .values(Params {
    //        id: params.new_id,
    //        name: &params.new_name.to_string(),
    //        password: &params.new_pass.to_string(),
    //    })
    //    .get_result(server.pool.get());
        //.returning(dsl::id);
    let result = html_routes::display_result_add(params.new_name.to_string());
    result
}
