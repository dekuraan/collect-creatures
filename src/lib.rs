use blake3::Hash;
pub use chrono::prelude::*;
pub use h3ron::H3Cell;
use h3ron::Index;
use rand::{prelude::StdRng, Rng, SeedableRng};

#[derive(bevy::prelude::Component, Debug)]
pub struct Spacetime {
    pub space: H3Cell,
    pub time: DateTime<Utc>,
}

impl Spacetime {
    pub fn new(space: H3Cell, date_time: DateTime<Utc>) -> Spacetime {
        Spacetime {
            space,
            time: date_time,
        }
    }
    pub fn hash(&self) -> Hash {
        let mut hasher = blake3::Hasher::new();
        hasher.update(&self.space.h3index().to_be_bytes());
        hasher.update(&self.time.date().year().to_be_bytes());
        hasher.update(&self.time.date().month().to_be_bytes());
        hasher.update(&self.time.date().day().to_be_bytes());
        hasher.update(&self.time.time().hour().to_be_bytes());
        hasher.update(&self.time.time().minute().to_be_bytes());
        hasher.finalize()
    }
}

#[cfg(test)]
mod tests {
    use crate::{get_creature_at_spacetime, Spacetime};
    use chrono::{TimeZone, Utc};
    use h3ron::H3Cell;
    use slippy_map_tiles::lat_lon_to_tile;

    #[test]
    fn scratchpad() {
        // let home = (38.876860, -77.154240);
        // let web_mercator = Proj::new("EPSG:3857").unwrap();
        // dbg!(web_mercator);
    }
}
use rand::distributions::WeightedIndex;
use rand::prelude::*;

pub fn get_creature_at_spacetime(st: &Spacetime) -> Option<&'static str> {
    let seed = st.hash();
    let mut rng = StdRng::from_seed(seed.into());
    let choices = [Some("pikachu.png"), Some("bulbasaur.png"), None];
    let weights = [1, 1, 1000];
    let dist = WeightedIndex::new(&weights).unwrap();
    choices[dist.sample(&mut rng)]
}
