mod database;

use actix_web::{HttpResponse, web};

pub fn mysql_router(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/hello").route(web::get().to(hello)))
        .service(web::resource("/databases/create").route(web::post().to(database::create_database)))
        .service(web::resource("/databases/delete").route(web::post().to(database::drop_database)))
    ;
}

async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello, Mysql")
}
