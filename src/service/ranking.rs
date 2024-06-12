use chrono::{DateTime, FixedOffset, Utc};
use serde_derive::{Deserialize, Serialize};
use crate::api::match_client::Match;
use crate::service::firebase_connector::Tip;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserRanking {
    pub user: Vec<UserRankingUser>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserRankingUser {
    pub name: String,
    pub position: isize,
    pub scoreSum: isize,
    pub tips: Vec<UserTip>
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserTip {
    pub matchId: isize,
    pub score: isize,
    pub tipTeam1: isize,
    pub tipTeam2: isize,
}

pub struct Ranking {}

impl Ranking {
    pub async fn get_user_ranking(matches: Vec<Match>, tips: Vec<Tip>) -> UserRanking {
        let mut user_ranking = UserRanking{ user: vec![] };

        for tip in tips {
            let index_option = user_ranking.user.iter().position(|r| r.name == tip.user);

            let mut user_ranking_user = UserRankingUser {
                name: tip.user,
                position: 0,
                scoreSum: 0,
                tips: vec![],
            };

            let mut already_exists = false;
            if index_option.is_some() {
                already_exists = true;
            }

            let found_match_option = matches.iter().find(|s| s.id == tip.id);

            if found_match_option.is_some() {
                let found_match = found_match_option.unwrap();
                let match_start_date_time = found_match.utcDate.parse::<DateTime<FixedOffset>>().unwrap();
                let now_date_time = Utc::now().with_timezone(&FixedOffset::west_opt(0).unwrap());

                if match_start_date_time.timestamp() < now_date_time.timestamp() {
                    let home_score = found_match.score.fullTime.home;
                    let away_score = found_match.score.fullTime.away;

                    let mut match_score: isize = 0;
                    if home_score.unwrap() == tip.score1 && away_score.unwrap() == tip.score2 {
                        // exactly right
                        match_score = 3;
                    } else if (home_score.unwrap() - tip.score1) == (away_score.unwrap() - tip.score2) {
                        // correct difference
                        match_score = 2;
                    } else if (home_score > away_score && tip.score1 > tip.score2) || (home_score < away_score && tip.score1 < tip.score2) {
                        // correct winner
                        match_score = 1;
                    }

                    if already_exists {
                        let index = index_option.unwrap();
                        user_ranking.user[index].tips.push(
                            UserTip {
                                matchId: tip.id,
                                score: match_score,
                                tipTeam1: tip.score1,
                                tipTeam2: tip.score2,
                            }
                        );
                    } else {
                        user_ranking_user.tips.push(
                            UserTip {
                                matchId: tip.id,
                                score: match_score,
                                tipTeam1: tip.score1,
                                tipTeam2: tip.score2,
                            }
                        );

                        user_ranking.user.push(user_ranking_user);
                    }
                }
            }
        }

        set_sum_score(&mut user_ranking.user);
        sort_user(&mut user_ranking);
        add_position(&mut user_ranking.user);

        user_ranking
    }
}

fn set_sum_score(user_ranking_user: &mut Vec<UserRankingUser>) {
    for u in user_ranking_user {
        let mut sum = 0;
        for user_tip in &u.tips {
            sum += user_tip.score;
        }

        u.scoreSum = sum;
    }
}

fn sort_user(user_ranking: &mut UserRanking) {
    user_ranking.user.sort_by(|a, b| b.scoreSum.cmp(&a.scoreSum));
}

fn add_position(user_ranking_user: &mut Vec<UserRankingUser>) {
    for u in user_ranking_user.clone() {
        let index = user_ranking_user.iter().position(|r| r.name == u.name).unwrap();
        let user_ranking_user_clone = user_ranking_user.clone();
        let user = &mut user_ranking_user[index];

        if index as i32 == 0 {
            user.position = 1;
        } else {
            let user_in_front_of_u = &user_ranking_user_clone[index - 1];

            if user.scoreSum == user_in_front_of_u.scoreSum {
                user.position = user_in_front_of_u.position;
            } else {
                // user_in_front_of_u.position + 1; or index + 1
                user.position = user_in_front_of_u.position + 1;
            }
        }
    }
}
