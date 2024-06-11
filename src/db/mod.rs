use rusqlite::{Connection, Result as SqliteResult};
use serde::Serialize;
use std::env;
use dotenv::dotenv;

#[derive(Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Debug, Serialize)]
pub struct Tip {
    pub id: i32,
    pub user_id: i32,
    pub match_id: i32,
    pub score_home: i32,
    pub score_away: i32,
}

#[derive(Debug, Serialize)]
pub struct Game {
    pub id: i32,
    pub home_score: i32,
    pub away_score: i32,
}

pub fn establish_connection() -> SqliteResult<Connection> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Connection::open(database_url)
}

pub fn get_users() -> SqliteResult<Vec<User>> {
    let conn = establish_connection()?;

    let mut stmt = conn.prepare("SELECT id, username FROM user")?;

    let user_iter = stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
        })
    })?;

    let mut user_list = Vec::new();
    for user in user_iter {
        user_list.push(user?);
    }

    Ok(user_list)
}

pub fn get_tips() -> SqliteResult<Vec<Tip>> {
    let conn = establish_connection()?;

    let mut stmt = conn.prepare("SELECT id, user_id, match_id, score_home, score_away FROM tip")?;

    let tips_iter = stmt.query_map([], |row| {
        Ok(Tip {
            id: row.get(0)?,
            user_id: row.get(1)?,
            match_id: row.get(2)?,
            score_home: row.get(3)?,
            score_away: row.get(4)?,
        })
    })?;

    let mut tips_list = Vec::new();
    for tip in tips_iter {
        tips_list.push(tip?);
    }

    Ok(tips_list)
}

pub fn get_tips_by_user(user_id: i32) -> SqliteResult<Vec<Tip>> {
    let conn = establish_connection()?;

    let mut stmt = conn.prepare("SELECT id, user_id, match_id, score_home, score_away FROM tip WHERE user_id = ?1")?;

    let tips_iter = stmt.query_map([user_id], |row| {
        Ok(Tip {
            id: row.get(0)?,
            user_id: row.get(1)?,
            match_id: row.get(2)?,
            score_home: row.get(3)?,
            score_away: row.get(4)?,
        })
    })?;

    let mut tips_list = Vec::new();
    for tip in tips_iter {
        tips_list.push(tip?);
    }

    Ok(tips_list)
}

pub fn get_past_games() -> SqliteResult<Vec<Game>> {
    let conn = establish_connection()?;


    let mut stmt = conn.prepare("SELECT id, home_score, away_score FROM match WHERE home_score >= 0 AND away_score >= 0")?;

    let game_iter = stmt.query_map([], |row| {
        Ok(Game {
            id: row.get(0)?,
            home_score: row.get(1)?,
            away_score: row.get(2)?,
        })
    })?;

    let mut game_list = Vec::new();
    for tip in game_iter {
        game_list.push(tip?);
    }

    Ok(game_list)
}

