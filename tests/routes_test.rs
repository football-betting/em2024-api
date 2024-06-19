use actix_web::{test, App};
use em2021_api::routes::{get_past_result_by_game_id, status};
use em2021_api::service::MatchInfo;
use std::env;
use actix_web::dev::ServiceResponse;

mod common;

#[actix_web::test]
async fn test_init_db() {

    let conn = common::setup();

    let mut stmt = conn.prepare("SELECT COUNT(*) FROM user").unwrap();
    let user_count: i32 = stmt.query_row([], |row| row.get(0)).unwrap();

    assert_eq!(user_count, 7);
}

#[actix_web::test]
async fn test_get_past_result_by_game_id() {
    let resp = get_response_by_url("/game/2").await;

    assert!(resp.status().is_success());

    let result: Vec<MatchInfo> = test::read_body_json(resp).await;

    assert!(!result.is_empty());

    assert_eq!(result.len(), 7);
    assert_eq!(result[0].match_id, "2".to_string());
    assert_eq!(result[0].user, "JohnDoe");
    assert_eq!(result[0].score, 0);

    assert_eq!(result[1].match_id, "2".to_string());
    assert_eq!(result[1].user, "ToniKroos");
    assert_eq!(result[1].score, 4);

    assert_eq!(result[2].match_id, "2".to_string());
    assert_eq!(result[2].team1.name, "Poland");
    assert_eq!(result[2].team2.name, "France");
    assert_eq!(result[2].score, 1);
    assert_eq!(result[2].tip_home, Some(2));
    assert_eq!(result[2].tip_away, Some(2));
    assert_eq!(result[2].score_home, Some(1));
    assert_eq!(result[2].score_away, Some(1));
}

#[actix_web::test]
async fn test_status() {
    let resp = get_response_by_url("/").await;

    assert!(resp.status().is_success());

    let result: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(result.get("status").unwrap().as_str().unwrap(), "works");
}


async fn get_response_by_url(url: &str) -> ServiceResponse {
    env::set_var("MODE", "test");

    let app = test::init_service(
        App::new()
            .service(get_past_result_by_game_id)
            .service(status)
    ).await;


    let req = test::TestRequest::get()
        .uri(url)
        .to_request();

    let resp = test::call_service(&app, req).await;
    resp
}
