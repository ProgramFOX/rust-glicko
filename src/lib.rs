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

const Q: f32 = 0.0057565;

fn g(rd: f32) -> f32 {
    (1f32 + 3f32 * Q.powi(2) * rd.powi(2) / std::f32::consts::PI.powi(2)).sqrt().recip()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
