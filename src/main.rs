use actix_web::{web,get, App, HttpResponse, HttpServer, Responder, Result as ActixResult};

mod db;

#[get("/users")]
async fn users() -> ActixResult<impl Responder> {
    match db::get_users() {
        Ok(user_list) => Ok(HttpResponse::Ok().json(user_list)),
        Err(e) => {
            eprintln!("Fehler beim Abrufen der Benutzer: {}", e);
            Ok(HttpResponse::InternalServerError().body("Fehler beim Abrufen der Benutzer"))
        }
    }
}

#[get("/tips")]
async fn tips() -> ActixResult<impl Responder> {
    match db::get_tips() {
        Ok(tips_list) => Ok(HttpResponse::Ok().json(tips_list)),
        Err(e) => {
            eprintln!("Fehler beim Abrufen der Benutzer: {}", e);
            Ok(HttpResponse::InternalServerError().body("Fehler beim Abrufen der Benutzer"))
        }
    }
}


#[get("/tips/{user_id}")]
async fn tips_by_user(user_id: web::Path<i32>) -> ActixResult<impl Responder> {
    match db::get_tips_by_user(user_id.into_inner()) {
        Ok(tips_list) => Ok(HttpResponse::Ok().json(tips_list)),
        Err(e) => {
            eprintln!("Fehler beim Abrufen der Tipps: {}", e);
            Ok(HttpResponse::InternalServerError().body("Fehler beim Abrufen der Tipps"))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(tips)
            .service(users)
            .service(tips_by_user)

    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
