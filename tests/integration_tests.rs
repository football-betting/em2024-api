mod common;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_users() {
        let conn = common::setup();

        let mut stmt = conn.prepare("SELECT COUNT(*) FROM user").unwrap();
        let user_count: i32 = stmt.query_row([], |row| row.get(0)).unwrap();

        assert_eq!(user_count, 7);
    }

    // #[test]
    // fn test_insert_games() {
    //    let conn = common::setup();
    //
    //     let mut stmt = conn.prepare("SELECT COUNT(*) FROM match").unwrap();
    //     let game_count: i32 = stmt.query_row([], |row| row.get(0)).unwrap();
    //
    //     assert_eq!(game_count, 5);
    // }
    //
    // #[test]
    // fn test_insert_tips() {
    //     let conn = common::setup();
    //
    //     let mut stmt = conn.prepare("SELECT COUNT(*) FROM tip").unwrap();
    //     let tip_count: i32 = stmt.query_row([], |row| row.get(0)).unwrap();
    //
    //     assert_eq!(tip_count, 11);
    // }
}
