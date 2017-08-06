#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
pub enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    fn numeric_value(&self) -> f32 {
        match *self {
            Outcome::Win => 1f32,
            Outcome::Draw => 0.5f32,
            Outcome::Loss => 0f32,
        }
    }
}

#[derive(Copy, Clone)]
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

    pub fn add_game(&mut self, game: RatedGame) {
        self.games.push(game);
    }

    fn d2(&self) -> f32 {
        let mut sum = 0f32;
        let r = self.player.rating;
        for game in &self.games {
            let r_j = game.opponent.rating;
            let rd_j = game.opponent.rd;

            let result_of_e = e(r, r_j, rd_j);
            sum += g(rd_j).powi(2) * result_of_e * (1f32 - result_of_e);
        }

        (Q.powi(2) * sum).recip()
    }

    pub fn calculate_new_rating(&self) -> RatedPlayer {
        let r = self.player.rating;
        let rd = self.player.rd;

        let mut sum = 0f32;
        for game in &self.games {
            let r_j = game.opponent.rating;
            let rd_j = game.opponent.rd;
            let s_j = game.outcome.numeric_value();

            sum += g(rd_j) * (s_j - e(r, r_j, rd_j));
        }

        let d_2 = self.d2();

        let new_r = r + (Q / (rd.powi(2).recip() + d_2.recip())) * sum;
        let new_rd = (rd.powi(2).recip() + d_2.recip()).recip().sqrt();

        RatedPlayer {
            rating: new_r,
            rd: new_rd,
        }
    }
}

#[derive(Copy, Clone)]
pub struct IndexedRatedPlayer {
    pub rating: f32,
    pub rd: f32,
    index: usize
}

impl IndexedRatedPlayer {
    fn without_index(&self) -> RatedPlayer {
        RatedPlayer {
            rating: self.rating,
            rd: self.rd,
        }
    }
}

#[derive(Default)]
pub struct RatingPeriod {
    players: Vec<IndexedRatedPlayer>,
    calculators: Vec<RatingCalculator>
}

impl RatingPeriod {
    pub fn new() -> RatingPeriod {
        RatingPeriod {
            players: vec![],
            calculators: vec![],
        }
    }

    pub fn add_player(&mut self, player: RatedPlayer) -> IndexedRatedPlayer {
        let index = self.players.len();
        self.calculators.push(RatingCalculator::for_player(player));
        let irp = IndexedRatedPlayer {
            rating: player.rating,
            rd: player.rd,
            index: index,
        };
        self.players.push(irp);
        irp
    }

    pub fn add_result(&mut self, winner: IndexedRatedPlayer, loser: IndexedRatedPlayer) {
        self.calculators[winner.index].add_game(RatedGame { outcome: Outcome::Win, opponent: loser.without_index() });
        self.calculators[loser.index].add_game(RatedGame { outcome: Outcome::Loss, opponent: winner.without_index() });
    }

    pub fn add_draw(&mut self, player1: IndexedRatedPlayer, player2: IndexedRatedPlayer) {
        self.calculators[player1.index].add_game(RatedGame { outcome: Outcome::Draw, opponent: player2.without_index() });
        self.calculators[player2.index].add_game(RatedGame { outcome: Outcome::Draw, opponent: player1.without_index() });
    }

    pub fn calculate_new_ratings(&self) -> Vec<RatedPlayer> {
        let mut result = vec![];
        for calculator in &self.calculators {
            result.push(calculator.calculate_new_rating());
        }
        result
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
