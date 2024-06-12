use std::error::Error;
use chrono::{DateTime};
use serde_derive::{Deserialize, Serialize};
use crate::db::{get_already_finished_matches, get_past_games, get_users};
use crate::service::{get_user_rating, UserRating};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Match {
    pub id: isize,
    pub utcDate: String,
    pub homeTeam: Team,
    pub awayTeam: Team,
    // pub score: Score,
    pub status: String,
    pub homeScore: Option<isize>,
    pub awayScore: Option<isize>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Team {
    pub id: Option<isize>,
    pub name: Option<String>,
    pub shortName: Option<String>,
    pub tla: Option<String>,
    pub flag: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Score {
    pub winner: Option<String>,
    pub duration: String,
    pub fullTime: ScoreDetail,
    pub halfTime: ScoreDetail,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ScoreDetail {
    pub home: Option<isize>,
    pub away: Option<isize>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DailyWinners {
    dailyWinners: Vec<DailyWinner>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct DailyWinner {
    date: String,
    user: Vec<String>,
    points: isize,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct AllDailies {
    dailies: Vec<Daily>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
struct Daily {
    date: String,
    dailyPoints: Vec<DailyPoints>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct DailyPoints {
    user: String,
    points: isize,
}

pub struct DailyWinnerService {}

impl DailyWinnerService {
    pub fn get_daily_winners() -> Result<DailyWinners, Box<dyn Error>> {
        let mut daily_winners: Vec<DailyWinner> = vec![];

        let finished_matches = get_already_finished_matches();

        let mut all_dailies = AllDailies { dailies: vec![] };
        for finished_match in finished_matches {
            let date = DateTime::from_timestamp(finished_match.utcDate.parse::<i64>().unwrap(), 0).unwrap().date_naive();

            let daily_index = all_dailies.dailies.iter().position(|p| p.date == date.to_string());

            let mut is_new_daily = true;

            let daily = &mut Daily {
                date: date.to_string(),
                dailyPoints: vec![],
            };

            if daily_index.is_some() {
                is_new_daily = false;
            };

            let ratings: Result<Vec<UserRating>, Box<dyn Error>> = get_user_rating(get_past_games().unwrap(), get_users().unwrap());

            for user_rating in ratings.unwrap() {
                let tip = user_rating.tips.iter().find(|u| u.match_id == finished_match.id.to_string());

                if tip.is_some() {
                    let points: isize = tip.unwrap().score as isize;

                    if is_new_daily {
                        let index = daily.dailyPoints.iter().position(|u| u.user == user_rating.name);

                        if index.is_some() {
                            daily.dailyPoints[index.unwrap()].points += points;
                        } else {
                            daily.dailyPoints.push(
                                DailyPoints {
                                    user: user_rating.name,
                                    points,
                                }
                            );
                        }
                    } else {
                        let index = all_dailies.dailies[daily_index.unwrap()].dailyPoints.iter().position(|u| u.user == user_rating.name);

                        if index.is_some() {
                            all_dailies.dailies[daily_index.unwrap()].dailyPoints[index.unwrap()].points += points;
                        } else {
                            all_dailies.dailies[daily_index.unwrap()].dailyPoints.push(
                                DailyPoints {
                                    user: user_rating.name,
                                    points,
                                }
                            );
                        }
                    }
                }
            }

            if is_new_daily {
                all_dailies.dailies.push(daily.clone());
            }
        }

        for mut daily in all_dailies.dailies {
            daily.dailyPoints.sort_by(|a, b| b.points.cmp(&a.points));

            let mut daily_winner = DailyWinner {
                date: daily.date.to_string(),
                user: vec![],
                points: 0,
            };

            let mut highest_score = 0;
            for daily_point in daily.dailyPoints {
                if highest_score == 0 {
                    highest_score = daily_point.points;
                    daily_winner.user.push(daily_point.user);
                } else if highest_score == daily_point.points {
                    daily_winner.user.push(daily_point.user);
                } else {
                    break;
                }
            }

            daily_winner.points = highest_score;

            if !daily_winner.user.is_empty() {
                daily_winners.push(daily_winner);
            }
        }

        Ok(
            DailyWinners {
                dailyWinners: daily_winners
            }
        )
    }
}