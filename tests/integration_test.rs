use actix_web::{test, App};
use em2021_api::routes::{get_past_result_by_game_id, rating, Response, status, user_by_id, UserResponse};
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
async fn test_get_user_rating() {
    let resp = get_response_by_url("/rating").await;

    assert!(resp.status().is_success());

    let result: Response = test::read_body_json(resp).await;

    assert_eq!(result.table.departments.len(), 2);
    assert_eq!(result.table.global.len(), 7);

    let department = result.table.departments.get("Langenfeld").unwrap();
    assert_eq!(department.len(), 4);
    assert_eq!(department[0].name, "ToniKroos");
    assert_eq!(department[0].department, "Langenfeld");

    let department = result.table.departments.get("London").unwrap();
    assert_eq!(department.len(), 3);
    assert_eq!(department[0].name, "RobbieFowler");
    assert_eq!(department[0].department, "London");

    let global = &result.table.global;
    assert_eq!(global[0].name, "ToniKroos");
    assert_eq!(global[0].department, "Langenfeld");
    assert_eq!(global[0].score_sum, 6);
    assert_eq!(global[0].position, 1);
    assert_eq!(global[0].sum_win_exact, 1);
    assert_eq!(global[0].sum_score_diff, 1);
    assert_eq!(global[0].sum_team, 0);
    assert_eq!(global[0].tips.len(), 0);

    assert_eq!(global[1].name, "JohnDoe");
    assert_eq!(global[1].department, "Langenfeld");
    assert_eq!(global[1].score_sum, 4);
    assert_eq!(global[1].position, 2);

    assert_eq!(global[2].name, "RobbieFowler");
    assert_eq!(global[2].department, "London");
    assert_eq!(global[2].score_sum, 4);
    assert_eq!(global[2].position, 2);

    assert_eq!(global[6].name, "SteveMcManaman");
    assert_eq!(global[6].position, 5);
}


#[actix_rt::test]
async fn test_user_by_id_returns_user_when_exists() {
    let resp = get_response_by_url("/user/2").await;

    assert!(resp.status().is_success());

    let result: UserResponse = test::read_body_json(resp).await;

    assert_eq!(result.data.user_id, 2);
    assert_eq!(result.data.name, "ToniKroos");
    assert_eq!(result.data.department, "Langenfeld");
    assert_eq!(result.data.score_sum, 6);
    assert_eq!(result.data.position, 1);
    assert_eq!(result.data.sum_win_exact, 1);
    assert_eq!(result.data.sum_team, 0);
    assert_eq!(result.data.tips.len(), 2);

    assert_eq!(result.data.tips[0].match_id, "2".to_string());
    assert_eq!(result.data.tips[0].team1.name, "Poland");
    assert_eq!(result.data.tips[0].team2.name, "France");

    assert_eq!(result.data.tips[1].score, 2);
}

#[actix_rt::test]
async fn user_by_id_returns_not_found_when_user_does_not_exist() {
    let resp = get_response_by_url("/user/99999").await;

    assert_eq!(resp.status(), 404);
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
            .service(user_by_id)
            .service(rating)
    ).await;


    let req = test::TestRequest::get()
        .uri(url)
        .to_request();

    let resp = test::call_service(&app, req).await;
    resp
}
