pub const GOOD_WEIGHT:f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

#[derive(Debug, PartialEq)]
pub enum Niceness {
    Nice(u32),
    Naughty,
}

pub struct Kid {
    pub name: String,
    pub niceness: Niceness,
}

impl Kid {
    pub fn new(name: String, good_deeds: u32, bad_deeds: u32) -> Self {
        let niceness = if Self::is_nice(good_deeds, bad_deeds) {
            Niceness::Nice(good_deeds)
        } else {
            Niceness::Naughty
        };

        Self{ name, niceness }
    }

    pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
        if good_deeds == 0 && bad_deeds == 0 {
            return false;
        }

        let ratio = (good_deeds as f32 * GOOD_WEIGHT) / ((good_deeds as f32 * GOOD_WEIGHT) + (bad_deeds as f32 * BAD_WEIGHT));

        ratio >= 0.75
    }

    pub fn parse_row(csv_row: &str) -> Result<Kid, &'static str> {
        let mut fields = csv_row.split(',');
        let name = fields.next().ok_or("Name not provided")?.to_string();
        let good_deeds: u32 = fields.next().ok_or("Number of good deeds not provided")?.parse().map_err(|_| "Value of good deeds is invalid")?;
        let bad_deeds:u32 = fields.next().ok_or("Number of bad deeds not provided")?.parse().map_err(|_| "Value of bad deeds is invalid")?;

        Ok(Self::new(name, good_deeds, bad_deeds))
    }
}

fn main() {

}