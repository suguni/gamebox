use crate::weapon::Weapon;

struct Combatant {
    name: String,
    hp: i32,
    max_hp: i32,
    ac: i32,
    attack_bonus: i32,
    weapon: Weapon,
}

impl Combatant {
    fn new(name: &str, hp: i32, max_hp: i32, ac: i32, attack_bonus: i32, weapon: Weapon) -> Self {
        Self {
            name: name.to_string(),
            hp,
            max_hp,
            ac,
            attack_bonus,
            weapon,
        }
    }

    fn take_damage(&mut self, damage: i32) {
        self.hp = i32::max(self.hp - damage, 0);
    }

    fn is_alive(&self) -> bool {
        self.hp > 0
    }
}

fn goblin(name: &str) -> Combatant {
    Combatant::new(name, 6, 6, 16, 6, Weapon::new("Dagger", 1, 6, 2))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_goblin() {
        // 생성
        let mut goblin = goblin("Goblin #1");
        assert_eq!(goblin.hp, 6);
        assert_eq!(goblin.ac, 16);

        // 피해/회복 (Character와 동일한 로직)
        goblin.take_damage(3);
        assert_eq!(goblin.hp, 3);

        // 생존 여부
        assert!(goblin.is_alive());
        goblin.take_damage(10);
        assert!(!goblin.is_alive());
    }
}
