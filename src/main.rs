use std::collections::{HashMap, HashSet};
use actix_web::{get, App, HttpResponse, HttpServer, Responder, Result as ActixResult, web};
use serde::Serialize;
use crate::service::{calculate_positions, MatchInfo, UserRating};

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

#[derive(Debug, Serialize)]
pub struct UserResponse {
    data: UserRating,
}

#[derive(Serialize)]
struct StatusResponse {
    status: String,
}

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

#[get("/user/{user_id}")]
async fn user_by_id(user_id: web::Path<i32>) -> ActixResult<impl Responder> {
    match service::get_user_rating(db::get_past_games().unwrap(),db::get_users().unwrap()) {
        Ok(mut user_rating_list) => {
            let user_id = user_id.into_inner();
            calculate_positions(&mut user_rating_list, false);
            let find_user = user_rating_list.iter().find(|user| user.user_id == user_id).cloned();

            let response = match find_user {
                Some(user) => UserResponse { data: user },
                None => return Ok(HttpResponse::NotFound().body("User not found")),
            };

            Ok(HttpResponse::Ok().json(response))
        },
        Err(e) => {
            eprintln!("Fehler beim Abrufen der Benutzer: {}", e);
            Ok(HttpResponse::InternalServerError().body("Fehler beim Abrufen der Benutzer"))
        }
    }
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

                calculate_positions(&mut department_users, true);
                department_ratings.insert(department, department_users);
            }

            calculate_positions(&mut user_rating_list, true);
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

#[get("/game/{game_id}")]
pub async fn get_past_result_by_game_id(game_id: web::Path<String>) -> ActixResult<impl Responder> {
    match service::get_user_rating(db::get_past_games().unwrap(),db::get_users().unwrap()) {
        Ok(user_rating_list) => {
            let game_id = game_id.into_inner();

            let tips_with_match_id: Vec<&MatchInfo> = user_rating_list.iter()
                .flat_map(|user_rating| &user_rating.tips)
                .filter(|tip| tip.match_id == game_id)
                .collect();

            Ok(HttpResponse::Ok().json(tips_with_match_id))
        },
        Err(e) => {
            eprintln!("Fehler beim Abrufen der Benutzer: {}", e);
            Ok(HttpResponse::InternalServerError().body("Fehler beim Abrufen der Benutzer"))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(status)
            .service(rating)
            .service(daily_winner)
            .service(user_by_id)
            .service(get_past_result_by_game_id)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
