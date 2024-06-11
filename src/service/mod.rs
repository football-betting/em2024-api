use serde::Serialize;
use crate::db::{Game, get_tips_by_user, Tip, User};
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct UserRating {
    name: String,
    user_id: i32,
    position: i32,
    score_sum: i32,
    sum_win_exact: i32,
    sum_score_diff: i32,
    sum_team: i32,
    extra_point: i32,
    tips: Vec<MatchInfo>,
}

#[derive(Debug, Serialize)]
pub struct MatchInfo {
    match_id: String,
    user: String,
    user_id: i32,
    score: i32,
    team1: String,
    team2: String,
    tip_home: Option<i32>,
    tip_away: Option<i32>,
    score_home: Option<i32>,
    score_away: Option<i32>,
}

struct ScoreConfig;
impl ScoreConfig {
    pub const NO_WIN_TEAM: i32 = 0;
    pub const WIN_EXACT: i32 = 4;
    pub const WIN_SCORE_DIFF: i32 = 2;
    pub const WIN_TEAM: i32 = 1;
}

pub fn get_user_rating(games: Vec<Game>, users: Vec<User>) -> Result<Vec<UserRating>, Box<dyn std::error::Error>> {
    let mut user_rating_list = Vec::new();

    for user in &users {
        let mut user_rating = UserRating {
            name: user.username.clone(),
            user_id: user.id.clone(),
            position: 0,
            score_sum: ScoreConfig::NO_WIN_TEAM,
            sum_win_exact: 0,
            sum_score_diff: 0,
            sum_team: 0,
            extra_point: 0,
            tips: Vec::new(),
        };
        let tips_by_user: HashMap<i32, Tip> = get_tips_by_user(user.id)?
            .into_iter()
            .map(|tip| (tip.match_id, tip))
            .collect();

        for game in &games {

            let mut match_info = MatchInfo {
                match_id: game.id.to_string(),
                user: user.username.clone(),
                user_id: user.id.clone(),
                score: 0,
                team1: "team1".to_string(),
                team2: "team2".to_string(),
                tip_home: None,
                tip_away: None,
                score_home: Some(game.home_score),
                score_away: Some(game.away_score),
            };

            if let Some(tip) = tips_by_user.get(&game.id) {
                match_info.tip_home = Some(tip.score_home);
                match_info.tip_away = Some(tip.score_away);

                calculate_score(&mut match_info);

                user_rating.score_sum += match_info.score;
                if match_info.score == ScoreConfig::WIN_EXACT {
                    user_rating.sum_win_exact += 1;
                } else if match_info.score == ScoreConfig::WIN_SCORE_DIFF {
                    user_rating.sum_score_diff += 1;
                } else if match_info.score == ScoreConfig::WIN_TEAM {
                    user_rating.sum_team += 1;
                }
            }

            user_rating.tips.push(match_info);
        }
        user_rating_list.push(user_rating);
    }

    calculate_positions(&mut user_rating_list);

    Ok(user_rating_list)
}

pub fn calculate_positions(user_rating_list: &mut Vec<UserRating>) {
    user_rating_list.sort_by(|a, b| b.score_sum.cmp(&a.score_sum));

    let mut position = 0;
    let mut last_point = -1;
    let mut position_for_frontend = 0;

    for user_rating in user_rating_list {
        position += 1;
        if user_rating.score_sum != last_point {
            position_for_frontend = position;
        }

        user_rating.position = position_for_frontend;

        last_point = user_rating.score_sum;
    }
}

fn calculate_score(match_info: &mut MatchInfo) {
    if let (Some(score_home), Some(score_away), Some(tip_home), Some(tip_away)) =
        (match_info.score_home, match_info.score_away, match_info.tip_home, match_info.tip_away) {

        if (score_home > score_away && tip_home > tip_away) || (score_home < score_away && tip_home < tip_away) {
            match_info.score = ScoreConfig::WIN_TEAM;
        }

        if score_home - score_away == tip_home - tip_away {
            if score_home == score_away {
                match_info.score = ScoreConfig::WIN_TEAM;
            } else {
                match_info.score = ScoreConfig::WIN_SCORE_DIFF;
            }
        }

        if score_home == tip_home && score_away == tip_away {
            match_info.score = ScoreConfig::WIN_EXACT;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_calculate_positions() {
            let mut user_rating_list = vec![
                UserRating {
                    name: "jahnedoe".to_string(),
                    score_sum: 2,
                    user_id: 1,position: 0,sum_win_exact: 0, sum_score_diff: 0, sum_team: 0, extra_point: 0, tips: Vec::new(),
                },
                UserRating {
                    name: "ninja".to_string(),
                    score_sum: 5,
                    user_id: 1,position: 0,sum_win_exact: 0, sum_score_diff: 0, sum_team: 0, extra_point: 0, tips: Vec::new(),
                },
                UserRating {
                    name: "babo".to_string(),
                    score_sum: 10,
                    user_id: 1,position: 0,sum_win_exact: 0, sum_score_diff: 0, sum_team: 0, extra_point: 0, tips: Vec::new(),
                },
                UserRating {
                    name: "abdul".to_string(),
                    score_sum: 9,
                    user_id: 1,position: 0,sum_win_exact: 0, sum_score_diff: 0, sum_team: 0, extra_point: 0, tips: Vec::new(),
                },
                UserRating {
                    name: "rockstar".to_string(),
                    score_sum: 5,
                    user_id: 1,position: 0,sum_win_exact: 0, sum_score_diff: 0, sum_team: 0, extra_point: 0, tips: Vec::new(),
                },
                UserRating {
                    name: "theBest".to_string(),
                    score_sum: 8,
                    user_id: 1,position: 0,sum_win_exact: 0, sum_score_diff: 0, sum_team: 0, extra_point: 0, tips: Vec::new(),
                },
                UserRating {
                    name: "johndoe".to_string(),
                    score_sum: 9,
                    user_id: 1,position: 0,sum_win_exact: 0, sum_score_diff: 0, sum_team: 0, extra_point: 0, tips: Vec::new(),
                },
            ];

            calculate_positions(&mut user_rating_list);

            assert_eq!(user_rating_list[0].position, 1);
            assert_eq!(user_rating_list[0].name, "babo");
            assert_eq!(user_rating_list[1].position, 2);
            assert_eq!(user_rating_list[1].name, "abdul");
            assert_eq!(user_rating_list[2].position, 2);
            assert_eq!(user_rating_list[2].name, "johndoe");
            assert_eq!(user_rating_list[3].position, 4);
            assert_eq!(user_rating_list[3].name, "theBest");
            assert_eq!(user_rating_list[4].position, 5);
            assert_eq!(user_rating_list[4].name, "ninja");
            assert_eq!(user_rating_list[5].position, 5);
            assert_eq!(user_rating_list[5].name, "rockstar");
            assert_eq!(user_rating_list[6].position, 7);
            assert_eq!(user_rating_list[6].name, "jahnedoe");
        }
    }

    #[test]
    fn test_calculate_positions_when_two_first_place() {
        let mut user_rating_list = vec![
            UserRating {
                name: "jahnedoe".to_string(),
                score_sum: 8,
                user_id: 1,position: 0,sum_win_exact: 0, sum_score_diff: 0, sum_team: 0, extra_point: 0, tips: Vec::new(),
            },
            UserRating {
                name: "ninja".to_string(),
                score_sum: 10,
                user_id: 1,position: 0,sum_win_exact: 0, sum_score_diff: 0, sum_team: 0, extra_point: 0, tips: Vec::new(),
            },
            UserRating {
                name: "babo".to_string(),
                score_sum: 10,
                user_id: 1,position: 0,sum_win_exact: 0, sum_score_diff: 0, sum_team: 0, extra_point: 0, tips: Vec::new(),
            },
            UserRating {
                name: "abdul".to_string(),
                score_sum: 9,
                user_id: 1,position: 0,sum_win_exact: 0, sum_score_diff: 0, sum_team: 0, extra_point: 0, tips: Vec::new(),
            },
            UserRating {
                name: "rockstar".to_string(),
                score_sum: 5,
                user_id: 1,position: 0,sum_win_exact: 0, sum_score_diff: 0, sum_team: 0, extra_point: 0, tips: Vec::new(),
            },
            UserRating {
                name: "theBest".to_string(),
                score_sum: 5,
                user_id: 1,position: 0,sum_win_exact: 0, sum_score_diff: 0, sum_team: 0, extra_point: 0, tips: Vec::new(),
            },
            UserRating {
                name: "johndoe".to_string(),
                score_sum: 9,
                user_id: 1,position: 0,sum_win_exact: 0, sum_score_diff: 0, sum_team: 0, extra_point: 0, tips: Vec::new(),
            },
        ];

        calculate_positions(&mut user_rating_list);

        assert_eq!(user_rating_list[0].position, 1);
        assert_eq!(user_rating_list[0].name, "ninja");
        assert_eq!(user_rating_list[1].position, 1);
        assert_eq!(user_rating_list[1].name, "babo");
        assert_eq!(user_rating_list[2].position, 3);
        assert_eq!(user_rating_list[2].name, "abdul");
        assert_eq!(user_rating_list[3].position, 3);
        assert_eq!(user_rating_list[3].name, "johndoe");
        assert_eq!(user_rating_list[4].position, 5);
        assert_eq!(user_rating_list[4].name, "jahnedoe");
        assert_eq!(user_rating_list[5].position, 6);
        assert_eq!(user_rating_list[5].name, "rockstar");
        assert_eq!(user_rating_list[6].position, 6);
        assert_eq!(user_rating_list[6].name, "theBest");
    }

    #[rstest]
    #[case(1, 2, 1, 2, ScoreConfig::WIN_EXACT)]
    #[case(2, 1, 2, 1, ScoreConfig::WIN_EXACT)]
    #[case(2, 0, 2, 0, ScoreConfig::WIN_EXACT)]
    #[case(0, 2, 0, 2, ScoreConfig::WIN_EXACT)]
    #[case(2, 2, 2, 2, ScoreConfig::WIN_EXACT)]
    #[case(2, 1, 0, 1, ScoreConfig::NO_WIN_TEAM)]
    #[case(1, 3, 3, 2, ScoreConfig::NO_WIN_TEAM)]
    #[case(0, 0, 2, 0, ScoreConfig::NO_WIN_TEAM)]
    #[case(0, 1, 0, 0, ScoreConfig::NO_WIN_TEAM)]
    #[case(1, 3, 2, 4, ScoreConfig::WIN_SCORE_DIFF)]
    #[case(4, 2, 3, 1, ScoreConfig::WIN_SCORE_DIFF)]
    #[case(1, 0, 2, 1, ScoreConfig::WIN_SCORE_DIFF)]
    #[case(1, 2, 0, 1, ScoreConfig::WIN_SCORE_DIFF)]
    #[case(3, 3, 0, 0, ScoreConfig::WIN_TEAM)]
    #[case(3, 3, 4, 4, ScoreConfig::WIN_TEAM)]
    #[case(1, 3, 1, 2, ScoreConfig::WIN_TEAM)]
    #[case(2, 1, 3, 1, ScoreConfig::WIN_TEAM)]
    #[case(1, 0, 2, 0, ScoreConfig::WIN_TEAM)]
    #[case(0, 5, 0, 2, ScoreConfig::WIN_TEAM)]
    #[case(2, 3, 2, 5, ScoreConfig::WIN_TEAM)]
    fn test_calculate_score(#[case] score_home: i32, #[case] score_away: i32, #[case] tip_home: i32, #[case] tip_away: i32, #[case] expected: i32) {
        let mut match_info = MatchInfo {
            match_id: "1".to_string(),
            user: "user".to_string(),
            user_id: 1,
            score: 0,
            team1: "team1".to_string(),
            team2: "team2".to_string(),
            tip_home: Some(tip_home),
            tip_away: Some(tip_away),
            score_home: Some(score_home),
            score_away: Some(score_away),
        };

        calculate_score(&mut match_info);

        assert_eq!(match_info.score, expected, "Error: score_home: {}, score_away: {}, tip_home: {}, tip_away: {}", score_home, score_away, tip_home, tip_away);
    }

    #[rstest]
    #[case(Some(0), Some(1), None, None, ScoreConfig::NO_WIN_TEAM)]
    #[case(Some(0), Some(0), None, None, ScoreConfig::NO_WIN_TEAM)]
    #[case(Some(1), Some(0), None, None, ScoreConfig::NO_WIN_TEAM)]
    #[case(None, None, Some(1), Some(0), ScoreConfig::NO_WIN_TEAM)]
    #[case(None, None, Some(0), Some(0), ScoreConfig::NO_WIN_TEAM)]
    #[case(None, None, Some(0), Some(1), ScoreConfig::NO_WIN_TEAM)]
    fn test_calculate_score_with_none(#[case] score_home: Option<i32>, #[case] score_away: Option<i32>, #[case] tip_home: Option<i32>, #[case] tip_away: Option<i32>, #[case] expected: i32) {
        let mut match_info = MatchInfo {
            match_id: "1".to_string(),
            user: "user".to_string(),
            user_id: 1,
            score: 0,
            team1: "team1".to_string(),
            team2: "team2".to_string(),
            tip_home: tip_home,
            tip_away: tip_away,
            score_home: score_home,
            score_away: score_away,
        };

        calculate_score(&mut match_info);

        assert_eq!(match_info.score, expected);
    }
}