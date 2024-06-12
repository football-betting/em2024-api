use actix_web::{web,get, App, HttpResponse, HttpServer, Responder, Result as ActixResult};

mod db;
mod service;

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

#[get("/rating")]
async fn rating() -> ActixResult<impl Responder> {
    match service::get_user_rating(db::get_past_games().unwrap(),db::get_users().unwrap()) {
        Ok(mut user_rating_list) => {
            service::calculate_positions(&mut user_rating_list);
            Ok(HttpResponse::Ok().json(user_rating_list))
        },
        Err(e) => {
            eprintln!("Fehler beim Abrufen der Benutzer: {}", e);
            Ok(HttpResponse::InternalServerError().body("Fehler beim Abrufen der Benutzer"))
        }
    }
}

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

#[get("/games")]
async fn games() -> ActixResult<impl Responder> {
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

#[get("/game")]
async fn game() -> ActixResult<impl Responder> {
    match db::get_past_games() {
        Ok(game_list) => Ok(HttpResponse::Ok().json(game_list)),
        Err(e) => {
            eprintln!("Fehler beim Abrufen der Games: {}", e);
            Ok(HttpResponse::InternalServerError().body("Fehler beim Abrufen der Games"))
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
            .service(game)
            .service(rating)
            .service(daily_winner)

    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
