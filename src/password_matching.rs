use actix_web::{
    //middleware, 
    web, 
    //App, 
    //HttpRequest, 
    HttpResponse, 
    //HttpServer, 
    //Responder, 
    Result,
};

//use serde_derive::{Serialize, Deserialize};

use crate::html_routes::*;
use crate::model::SecureParams;
use crate::model::MyParams;

pub fn pass_maching(from_form_path: String) -> bool {
    let password = "".to_string();
    if password == from_form_path { true }
    else{ false }
}

pub fn maching_securepass(params: web::Form<SecureParams>) -> Result<HttpResponse> {
    let secure_from_form = &params.secure_password;
    let matching_flag = pass_maching(secure_from_form.to_string());
    let result
        = if matching_flag { add_new_user() }
        else{ secure_form() };
    result
}

pub fn get_login_params(params: web::Form<MyParams>) -> Result<HttpResponse> {
    let path_from_form = &params.pass;
    let matching_flag = pass_maching(path_from_form.to_string());
    let result
        = if matching_flag { result_matching_password() }
        else{ index() };
    result
}
