pub struct RatedPlayer {
    pub rating: f32,
    pub rd: f32,
}

impl RatedPlayer {
    pub fn from_rating_and_rd(rating: f32, rd: f32) -> RatedPlayer {
        RatedPlayer {
            rating: rating,
            rd: rd,
        }
    }

    pub fn from_rating_and_rd_and_inactivity(rating: f32, rd: f32, c: f32, t: i32) -> RatedPlayer {
        let new_rd = (rd.powi(2) + c.powi(2) * (t as f32)).sqrt();
        let new_rd = if new_rd > 350f32 { 350f32 } else { new_rd };
        RatedPlayer {
            rating: rating,
            rd: new_rd,
        }
    }
}

const Q: f32 = 0.0057564627324851142100449786;

fn g(rd: f32) -> f32 {
    (1f32 + 3f32 * Q.powi(2) * rd.powi(2) / std::f32::consts::PI.powi(2)).sqrt().recip()
}

fn e(r: f32, r_j: f32, rd_j: f32) -> f32 {
    (1f32 + 10f32.powf(-g(rd_j)*(r - r_j)/400f32)).recip()
}

pub enum Outcome {
    Win,
    Draw,
    Loss,
}

pub struct RatedGame {
    pub outcome: Outcome,
    pub opponent: RatedPlayer,
}

pub struct RatingCalculator {
    player: RatedPlayer,
    games: Vec<RatedGame>,
}

impl RatingCalculator {
    pub fn for_player(player: RatedPlayer) -> RatingCalculator {
        RatingCalculator {
            player: player,
            games: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
