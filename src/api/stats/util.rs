use crate::error::DieselError;
use crate::models::{Game, User};
use crate::util::function;
use anyhow::Result;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct StatsResponse {
    pub highest_attack_score: i32,
    pub highest_defense_score: i32,
    pub rating: i32,
    pub highest_rating: i32,
    pub position_in_leaderboard: i32,
    pub no_of_robots_killed: i32,
    pub no_of_robots_got_killed: i32,
    pub no_of_emps_used: i32,
    pub total_damage_defense: i32,
    pub total_damage_attack: i32,
    pub no_of_attackers_suicided: i32,
    pub no_of_attacks: i32,
    pub no_of_defenses: i32,
}

pub fn fetch_user(conn: &PgConnection, player_id: i32) -> Result<User> {
    use crate::schema::user;
    Ok(user::table
        .filter(user::id.eq(player_id))
        .first::<User>(conn)
        .map_err(|err| DieselError {
            table: "user",
            function: function!(),
            error: err,
        })?)
}

pub fn fetch_all_user(conn: &PgConnection) -> Result<Vec<User>> {
    use crate::schema::user;
    Ok(user::table
        .order_by(user::overall_rating.desc())
        .load::<User>(conn)
        .map_err(|err| DieselError {
            table: "user",
            function: function!(),
            error: err,
        })?)
}

pub fn fetch_attack_game(conn: &PgConnection, player_id: i32) -> Result<Vec<Game>> {
    use crate::schema::game;
    Ok(game::table
        .filter(game::attack_id.eq(player_id))
        .order_by(game::attack_score.desc())
        .load::<Game>(conn)
        .map_err(|err| DieselError {
            table: "game",
            function: function!(),
            error: err,
        })?)
}

pub fn fetch_defense_game(conn: &PgConnection, player_id: i32) -> Result<Vec<Game>> {
    use crate::schema::game;
    Ok(game::table
        .filter(game::defend_id.eq(player_id))
        .order_by(game::defend_score.desc())
        .load::<Game>(conn)
        .map_err(|err| DieselError {
            table: "game",
            function: function!(),
            error: err,
        })?)
}

pub fn make_response(
    user: &User,
    attack_game: &[Game],
    defense_game: &[Game],
    users: &[User],
) -> Result<StatsResponse> {
    let mut stats = StatsResponse {
        highest_attack_score: 0,
        highest_defense_score: 0,
        rating: user.overall_rating,
        highest_rating: user.highest_rating,
        position_in_leaderboard: 0,
        no_of_robots_killed: 0,
        no_of_robots_got_killed: 0,
        no_of_emps_used: 0,
        total_damage_defense: 0,
        total_damage_attack: 0,
        no_of_attackers_suicided: 0,
        no_of_attacks: attack_game.len() as i32,
        no_of_defenses: defense_game.len() as i32,
    };

    if !attack_game.is_empty() {
        stats.highest_attack_score = attack_game[0].attack_score;
        for attack in attack_game {
            stats.total_damage_attack += attack.damage_done;
            stats.no_of_robots_killed += attack.robots_destroyed;
            stats.no_of_emps_used += attack.emps_used;
            if !attack.is_attacker_alive {
                stats.no_of_attackers_suicided += 1;
            }
        }
    }
    if !defense_game.is_empty() {
        stats.highest_defense_score = defense_game[0].defend_score;
        for defend in defense_game {
            stats.total_damage_defense += defend.damage_done;
            stats.no_of_robots_got_killed += defend.robots_destroyed;
        }
    }
    if !users.is_empty() {
        for (i, u) in users.iter().enumerate() {
            if user.id == u.id {
                stats.position_in_leaderboard = i as i32;
                break;
            }
        }
        stats.position_in_leaderboard += 1;
    }
    Ok(stats)
}