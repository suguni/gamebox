use rand::{Rng, RngExt};

pub struct Weapon {
    name: String,
    num_of_dice: u8,
    faces: u8,
    bonus: i32,
}

impl Weapon {
    pub fn new(name: &str, num_of_dice: u8, faces: u8, bonus: i32) -> Self {
        Self {
            name: name.to_string(),
            num_of_dice,
            faces,
            bonus,
        }
    }

    fn roll_damage(&self, rng: &mut impl Rng) -> i32 {
        let mut damage = 0;
        for _ in 0..self.num_of_dice {
            damage += rng.random_range(1..=self.faces) as u32;
        }
        (damage as i32) + self.bonus
    }
}

#[cfg(test)]
mod tests {

    use rand::{rngs::StdRng, SeedableRng};

    use super::*;

    #[test]
    fn test_weapon() {
        let mut rng = StdRng::seed_from_u64(42);
        let w = Weapon::new("Longsword", 1, 8, 0);
        let d = w.roll_damage(&mut rng);
        assert!((1..=8).contains(&d));
        assert_eq!(d, 2);
    }
}
