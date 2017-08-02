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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
