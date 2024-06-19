use std::collections::HashMap;
use actix_web::{get, HttpResponse, Responder, Result as ActixResult, web};
use serde_derive::{Deserialize, Serialize};
use crate::{db, service};
use crate::service::{calculate_positions, MatchInfo, UserRating};

#[derive(Debug, Serialize, Deserialize)]
struct StatusResponse {
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RatingResponse {
    global: Vec<UserRating>,
    departments: HashMap<String, Vec<UserRating>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    table: RatingResponse,
    daily_winner: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    data: UserRating,
}

#[get("/user/{user_id}")]
pub async fn user_by_id(user_id: web::Path<i32>) -> ActixResult<impl Responder> {
    match service::get_user_rating(db::get_past_games().unwrap(),db::get_users().unwrap()) {
        Ok(mut user_rating_list) => {
            let user_id = user_id.into_inner();
            calculate_positions(&mut user_rating_list, false);
            let find_user = user_rating_list.iter().find(|user| user.user_id == user_id).cloned();

            let response = match find_user {
                Some(mut user) => {
                    user.tips.sort_by(|a, b| b.date.cmp(&a.date));
                    UserResponse { data: user }
                },
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

#[get("/game/{game_id}")]
pub async fn get_past_result_by_game_id(game_id: web::Path<String>) -> ActixResult<impl Responder> {
    match crate::service::get_user_rating(crate::db::get_past_games().unwrap(), crate::db::get_users().unwrap()) {
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

#[get("/")]
pub async fn status() -> ActixResult<impl Responder> {
    let response = StatusResponse {
        status: String::from("works"),
    };

    Ok(HttpResponse::Ok().json(response))
}