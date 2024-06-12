use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use rusqlite::{params, Connection, Result};
use serde::Serialize;
use serde_json;

#[derive(Debug)]
pub struct User {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub department: String,
    pub winner: String,
    pub secret_winner: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Team {
    pub name: String,
    pub tla: String,
}

#[derive(Debug)]
pub struct Game {
    pub id: i32,
    pub home_team: String,
    pub away_team: String,
    pub status: String,
    pub utc_date: u64,
    pub home_score: Option<i32>,
    pub away_score: Option<i32>,
}

#[derive(Debug)]
pub struct Tip {
    pub user_id: i32,
    pub match_id: i32,
    pub date: u64,
    pub score_home: i32,
    pub score_away: i32,
}

pub fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS user (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT NOT NULL,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            username TEXT NOT NULL,
            department TEXT NOT NULL,
            winner TEXT NOT NULL,
            secret_winner TEXT NOT NULL
        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS match (
            id INTEGER PRIMARY KEY,
            homeTeam he NOT NULL,
            awayTeam TEXT NOT NULL,
            status TEXT NOT NULL,
            utcDate INTEGER NOT NULL,
            homeScore INTEGER,
            awayScore INTEGER
        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tip (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            match_id INTEGER NOT NULL,
            date INTEGER NOT NULL,
            score_home INTEGER NOT NULL,
            score_away INTEGER NOT NULL,
            FOREIGN KEY(user_id) REFERENCES user(id),
            FOREIGN KEY(match_id) REFERENCES game(id)
        )",
        [],
    )?;
    Ok(())
}

pub fn insert_users(conn: &Connection, users: &[User]) -> Result<()> {
    for user in users {
        conn.execute(
            "INSERT INTO user (email, first_name, last_name, username, department, winner, secret_winner) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![user.email, user.first_name, user.last_name, user.username, user.department, user.winner, user.secret_winner],
        )?;
    }
    Ok(())
}

pub fn insert_games(conn: &Connection, games: &[Game]) -> Result<()> {
    for game in games {
        conn.execute(
            "INSERT INTO match (id, homeTeam, awayTeam, status, utcDate, homeScore, awayScore) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![game.id, game.home_team, game.away_team, game.status, game.utc_date, game.home_score, game.away_score],
        )?;
    }
    Ok(())
}

pub fn insert_tips(conn: &Connection, tips: &[Tip]) -> Result<()> {
    for tip in tips {
        conn.execute(
            "INSERT INTO tip (user_id, match_id, date, score_home, score_away) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![tip.user_id, tip.match_id, tip.date, tip.score_home, tip.score_away],
        )?;
    }
    Ok(())
}

pub fn setup() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    //let conn = Connection::open("/home/ninja/workspace/github/football-betting/em2021-api/database_test.db").unwrap();
    create_tables(&conn).unwrap();

    let users = vec![
        User {
            email: String::from("john@doe.com"),
            first_name: String::from("John"),
            last_name: String::from("Doe"),
            username: String::from("JohnDoe"),
            department: String::from("Langenfeld"),
            winner: String::from("DEU"),
            secret_winner: String::from("ENG"),
        },
        User {
            email: String::from("toni@kroos.de"),
            first_name: String::from("Toni"),
            last_name: String::from("Kroos"),
            username: String::from("ToniKroos"),
            department: String::from("Langenfeld"),
            winner: String::from("DEU"),
            secret_winner: String::from("FRA"),
        },
        User {
            email: String::from("philipp@lahm.de"),
            first_name: String::from("Philipp"),
            last_name: String::from("Lahm"),
            username: String::from("PhilippLahm"),
            department: String::from("Langenfeld"),
            winner: String::from("ESP"),
            secret_winner: String::from("ENG"),
        },
        User {
            email: String::from("lukas@podolski.pl"),
            first_name: String::from("Lukas"),
            last_name: String::from("Podolski"),
            username: String::from("LukasPodolski"),
            department: String::from("Langenfeld"),
            winner: String::from("POL"),
            secret_winner: String::from("DEU"),
        },
        User {
            email: String::from("robbie@fowler.com"),
            first_name: String::from("Robbie"),
            last_name: String::from("Fowler"),
            username: String::from("RobbieFowler"),
            department: String::from("London"),
            winner: String::from("NLD"),
            secret_winner: String::from("ENG"),
        },
        User {
            email: String::from("bobby@moore.com"),
            first_name: String::from("Bobby"),
            last_name: String::from("Moore"),
            username: String::from("BobbyMoore"),
            department: String::from("London"),
            winner: String::from("ENG"),
            secret_winner: String::from("DEU"),
        },
        User {
            email: String::from("steve@mcmanaman.com"),
            first_name: String::from("Steve"),
            last_name: String::from("McManaman"),
            username: String::from("SteveMcManaman"),
            department: String::from("London"),
            winner: String::from("FRA"),
            secret_winner: String::from("ENG"),
        },
    ];

    let lands: HashMap<&str, Team> = [
        ("en", Team { name: String::from("England"), tla: String::from("ENG") }),
        ("nl", Team { name: String::from("Netherlands"), tla: String::from("NED") }),
        ("pl", Team { name: String::from("Poland"), tla: String::from("POL") }),
        ("fr", Team { name: String::from("France"), tla: String::from("FRA") }),
        ("de", Team { name: String::from("Germany"), tla: String::from("GER") }),
        ("es", Team { name: String::from("Spain"), tla: String::from("ESP") }),
    ].iter().cloned().collect();

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    let games = vec![
        Game {
            id: 1,
            home_team: serde_json::to_string(&lands["de"].clone()).unwrap(),
            away_team: serde_json::to_string(&lands["es"].clone()).unwrap(),
            status: String::from("scheduled"),
            utc_date: now - 86400, // 1 Tag vorher
            home_score: Some(2),
            away_score: Some(0),
        },
        Game {
            id: 2,
            home_team: serde_json::to_string(&lands["pl"].clone()).unwrap(),
            away_team: serde_json::to_string(&lands["fr"].clone()).unwrap(),
            status: String::from("scheduled"),
            utc_date: now - 1800, // 30 Minuten vorher
            home_score: Some(1),
            away_score: Some(1),
        },
        Game {
            id: 3,
            home_team: serde_json::to_string(&lands["en"].clone()).unwrap(),
            away_team: serde_json::to_string(&lands["nl"].clone()).unwrap(),
            status: String::from("scheduled"),
            utc_date: now + 3600, // 1 Stunde später
            home_score: None,
            away_score: None,
        },
        Game {
            id: 4,
            home_team: serde_json::to_string(&lands["fr"].clone()).unwrap(),
            away_team: serde_json::to_string(&lands["de"].clone()).unwrap(),
            status: String::from("scheduled"),
            utc_date: now + 86400, // 1 Tag später
            home_score: None,
            away_score: None,
        },
        Game {
            id: 5,
            home_team: serde_json::to_string(&lands["en"].clone()).unwrap(),
            away_team: serde_json::to_string(&lands["pl"].clone()).unwrap(),
            status: String::from("scheduled"),
            utc_date: now + (30 * 24 * 60 * 60), // 1 Monat später
            home_score: None,
            away_score: None,
        },
    ];

    insert_users(&conn, &users).unwrap();
    insert_games(&conn, &games).unwrap();

    let tpis = vec![
        Tip {
            user_id: 1,
            match_id: 1,
            date: now - 86400,
            score_home: 2,
            score_away: 0,
        },
        Tip {
            user_id: 1,
            match_id: 2,
            date: now - 86400,
            score_home: 1,
            score_away: 0,
        },
        Tip {
            user_id: 2,
            match_id: 1,
            date: now - 86400,
            score_home: 3,
            score_away: 1,
        },
        Tip {
            user_id: 2,
            match_id: 2,
            date: now - 86400,
            score_home: 1,
            score_away: 1,
        },
        Tip {
            user_id: 3,
            match_id: 1,
            date: now - 86400,
            score_home: 4,
            score_away: 1,
        },
        Tip {
            user_id: 3,
            match_id: 2,
            date: now - 86400,
            score_home: 2,
            score_away: 2,
        },
        Tip {
            user_id: 4,
            match_id: 1,
            date: now - 86400,
            score_home: 0,
            score_away: 1,
        },
        Tip {
            user_id: 4,
            match_id: 2,
            date: now - 86400,
            score_home: 2,
            score_away: 0,
        },
        Tip {
            user_id: 5,
            match_id: 1,
            date: now - 86400,
            score_home: 2,
            score_away: 0,
        },
        Tip {
            user_id: 5,
            match_id: 2,
            date: now - 86400,
            score_home: 0,
            score_away: 2,
        },
        Tip {
            user_id: 6,
            match_id: 1,
            date: now - 86400,
            score_home: 2,
            score_away: 4,
        },
    ];

    insert_tips(&conn, &tpis).unwrap();

    conn
}
