use actix_web::{
    //middleware, web, App, 
    //HttpRequest, 
    HttpResponse, 
    //HttpServer, 
    //Responder, 
    Result,
};

pub fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../text/hello.html")))
}

pub fn add_new_user() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../text/add_new_user.html")))
}

pub fn secure_form() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../text/get_secure_password.html")))
}

pub fn result_matching_password() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../text/result.html")))
}

pub fn display_result_add(param: String) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../text/result_add_new_user.html")))
}
