#[macro_use]
extern crate diesel;
extern crate serde_derive;
extern crate postgres;

use actix_web::{
    middleware, web, App, 
    //HttpRequest, 
    //HttpResponse, 
    HttpServer, 
    //Responder, 
    //Result,
};
use std::env;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
//use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
//use std::io::Read;

mod html_routes;
mod edit_database;
mod password_matching;
mod schema;
mod model;

//struct AppState { use 58 and 59 lines
//    foo: String,
//}

#[derive(Clone)]
pub struct Server {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl Server {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Server { pool }
    }
}

//fn app_config(config: &mut web::ServiceConfig) {//-> App<Server> {
//    //let var = server;
//    config.service(
//        web::scope("")
////            .data(AppState {
//                //foo: "bar".to_string(),
////            })
//            .service(web::resource("/").route(web::get().to(html_routes::index)))
//            .service(web::resource("/post1").route(web::post().to(password_matching::get_login_params)))
//            .service(web::resource("/secure").route(web::post().to(html_routes::secure_form)))
//            .service(web::resource("/maching_secure").route(web::post().to(password_matching::maching_securepass)))
//            .service(web::resource("/add_new_user").route(web::post().to(html_routes::add_new_user)))
//            .service(web::resource("/result_add").route(web::post().to(edit_database::add_new_user_system)))
//    );
//
//    //app
//}

fn main() -> std::io::Result<()> {
    //    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    //    builder
//        .set_private_key_file("key.pem", SslFiletype::PEM)
//        .unwrap();
//    builder.set_certificate_chain_file("cert.pem").unwrap();
//        
//    HttpServer::new(|| App::new().route("/", web::get().to(index)))
//        .bind_ssl("127.0.0.1:8088", builder)
//        .run()
//
dotenv::dotenv().ok();

   // let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
   // let manager = ConnectionManager::<PgConnection>::new(connspec);
   // let server_pool = Pool::builder()
   //     .build(manager)
   //     .expect("Failed to create pool.");
    let server = Server::new();
    HttpServer::new(move || {
        App::new()
            .data(server.clone())
            .wrap(middleware::Logger::default())

            .service(web::resource("/").route(web::get().to(html_routes::index)))
            .service(web::resource("/post1").route(web::post().to(password_matching::get_login_params)))
            .service(web::resource("/secure").route(web::post().to(html_routes::secure_form)))
            .service(web::resource("/maching_secure").route(web::post().to(password_matching::maching_securepass)))
            .service(web::resource("/add_new_user").route(web::post().to(html_routes::add_new_user)))
            .service(web::resource("/result_add").route(web::post().to(edit_database::add_new_user_system)))
            //.configure(app_config)
    })
    .bind("127.0.0.1:8080")?
    .run()
}
