use actix_web::{get, App, HttpResponse, HttpServer, Responder, Result as ActixResult};

mod db;
mod service;
mod routes;

#[get("/daily-winner")]
async fn daily_winner() -> ActixResult<impl Responder> {
    match service::daily_winner::DailyWinnerService::get_daily_winners() {
        Ok(v) => {
            Ok(HttpResponse::Ok().json(v))
        },
        Err(e) => {
            eprintln!("Fehler beim Abrufen der täglichen Gewinner: {}", e);
            Ok(HttpResponse::InternalServerError().body("Fehler beim Abrufen der täglichen Gewinner"))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(routes::status)
            .service(routes::rating)
            .service(daily_winner)
            .service(routes::user_by_id)
            .service(routes::get_past_result_by_game_id)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
