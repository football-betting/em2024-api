use std::collections::{HashMap, HashSet};
use actix_web::{get, App, HttpResponse, HttpServer, Responder, Result as ActixResult};
use serde::Serialize;
use crate::service::{calculate_positions, UserRating};

mod db;
mod service;

#[derive(Debug, Serialize)]
pub struct RatingResponse {
    global: Vec<UserRating>,
    departments: HashMap<String, Vec<UserRating>>,
}

#[derive(Debug, Serialize)]
pub struct Response {
    table: RatingResponse,
    daily_winner: Option<String>,
}

#[derive(Serialize)]
struct StatusResponse {
    status: String,
}

#[get("/rating")]
async fn rating() -> ActixResult<impl Responder> {
    match service::get_user_rating(db::get_past_games().unwrap(),db::get_users().unwrap()) {
        Ok(mut user_rating_list) => {
            let cloned_user_rating_list = user_rating_list.clone();
            let mut departments: HashSet<String> = HashSet::new();
            let mut department_ratings: HashMap<String, Vec<UserRating>> = HashMap::new();

            for user_rating in &cloned_user_rating_list {
                departments.insert(user_rating.department.clone());
            }

            for department in departments {
                let mut department_users: Vec<UserRating> = cloned_user_rating_list.iter().filter(|user| user.department == department).cloned().collect();

                calculate_positions(&mut department_users);
                department_ratings.insert(department, department_users);
            }

            calculate_positions(&mut user_rating_list);
            let rating_response = RatingResponse {
                global: user_rating_list,
                departments: department_ratings,
            };

            let response = Response {
                table: rating_response,
                daily_winner: None,
            };

            Ok(HttpResponse::Ok().json(response))
        },
        Err(e) => {
            eprintln!("Fehler beim Abrufen der Benutzer: {}", e);
            Ok(HttpResponse::InternalServerError().body("Fehler beim Abrufen der Benutzer"))
        }
    }
}

#[get("/")]
async fn status() -> ActixResult<impl Responder> {
    let response = StatusResponse {
        status: String::from("works"),
    };

    Ok(HttpResponse::Ok().json(response))
}

#[get("/game")]
async fn games() -> ActixResult<impl Responder> {
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
            .service(status)
            .service(rating)

    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
