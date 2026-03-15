fn ability_modifier(score: i32) -> i32 {
    (score - 10).div_euclid(2)
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Proficiency {
    Untrained,
    Trained,
    Expert,
    Master,
    Legendary,
}

fn proficiency_bonus(proficiency: Proficiency, level: i32) -> i32 {
    match proficiency {
        Proficiency::Untrained => 0,
        Proficiency::Trained => level + 2,
        Proficiency::Expert => level + 4,
        Proficiency::Master => level + 6,
        Proficiency::Legendary => level + 8,
    }
}

fn ac(dex_score: i32, armor_bonus: i32, proficiency_bonus: i32) -> i32 {
    let dex_mod = ability_modifier(dex_score);
    10 + dex_mod + armor_bonus + proficiency_bonus
}

fn max_hp(class_hp: i32, con_score: i32, level: i32) -> i32 {
    let con_mod = ability_modifier(con_score);
    (class_hp + con_mod) * level
}

#[derive(Debug, PartialEq, Eq)]
struct Attribute {
    strength: i32,
    dexterity: i32,
    constitution: i32,
    intelligence: i32,
    wisdom: i32,
    charisma: i32,
}

#[derive(Debug, PartialEq, Eq)]
struct Character {
    level: i32,
    class_hp: i32,
    hp: i32,
    stats: Attribute,
    armor: Defense,
    perception: Proficiency,
}

#[derive(Debug, PartialEq, Eq)]
struct Defense {
    armor_item: i32,
    armor_proficiency: Proficiency,
}

impl Character {
    fn new(class_hp: i32, stats: Attribute, armor: Defense, perception: Proficiency) -> Self {
        let level = 1;
        let hp = max_hp(class_hp, stats.constitution, level);
        Self {
            level,
            class_hp,
            hp,
            stats,
            armor,
            perception,
        }
    }

    fn ac(&self) -> i32 {
        let prof_bonus = proficiency_bonus(self.armor.armor_proficiency, self.level);
        ac(self.stats.dexterity, self.armor.armor_item, prof_bonus)
    }

    fn max_hp(&self) -> i32 {
        max_hp(self.class_hp, self.stats.constitution, self.level)
    }

    fn take_damage(&mut self, damage: i32) {
        self.hp = i32::max(self.hp - damage, 0);
    }

    fn heal(&mut self, heal: i32) {
        self.hp = i32::min(self.hp + heal, self.max_hp());
    }

    fn current_hp(&self) -> i32 {
        self.hp
    }

    fn perception_bonus(&self) -> i32 {
        ability_modifier(self.stats.wisdom) + proficiency_bonus(self.perception, self.level)
    }
}

#[cfg(test)]
mod tests {

    use super::Proficiency::*;
    use super::*;

    #[test]
    fn test_ability_modifier() {
        assert_eq!(ability_modifier(10), 0); // (10-10)/2
        assert_eq!(ability_modifier(12), 1); // (12-10)/2
        assert_eq!(ability_modifier(8), -1); // (8-10)/2
        assert_eq!(ability_modifier(1), -5); // 최솟값
        assert_eq!(ability_modifier(20), 5); // 최댓값 (1레벨 기준)
    }

    #[test]
    fn test_proficiency_bonus() {
        assert_eq!(proficiency_bonus(Untrained, 1), 0);
        assert_eq!(proficiency_bonus(Trained, 1), 3); // 1+2
        assert_eq!(proficiency_bonus(Trained, 3), 5); // 3+2
        assert_eq!(proficiency_bonus(Expert, 3), 7); // 3+4
        assert_eq!(proficiency_bonus(Master, 5), 11); // 5+6
        assert_eq!(proficiency_bonus(Legendary, 5), 13); // 5+8
    }

    #[test]
    fn test_ac() {
        // AC = 10 + DEX수정값 + 방어구보너스 + 숙련도보너스
        assert_eq!(ac(14, 2, 3), 17); // 10 + 2 + 2 + 3
        assert_eq!(ac(10, 0, 0), 10); // 최소 AC
    }

    #[test]
    fn test_max_hp() {
        assert_eq!(max_hp(10, 12, 1), 11); // (10+1) * 1
        assert_eq!(max_hp(10, 12, 3), 33); // (10+1) * 3
        assert_eq!(max_hp(8, 8, 1), 7); // (8-1) * 1
    }

    fn fighter() -> Character {
        Character::new(
            10,
            Attribute {
                strength: 16,
                dexterity: 12,
                constitution: 14,
                intelligence: 10,
                wisdom: 12,
                charisma: 10,
            },
            Defense {
                armor_item: 4,
                armor_proficiency: Proficiency::Trained,
            },
            Proficiency::Trained,
        )
    }

    #[test]
    fn test_1_lvl_character_create() {
        let fighter = fighter();

        assert_eq!(fighter.level, 1);
        assert_eq!(fighter.ac(), 18);
        assert_eq!(fighter.max_hp(), 12);
        assert_eq!(fighter.current_hp(), fighter.max_hp());
    }

    #[test]
    fn test_character_damage_hp() {
        let mut fighter = fighter();

        fighter.take_damage(5);
        assert_eq!(fighter.current_hp(), 7);

        fighter.take_damage(10); // 초과 피해
        assert_eq!(fighter.current_hp(), 0); // 0 아래로 내려가지 않음

        fighter.heal(3);
        assert_eq!(fighter.current_hp(), 3);
    }

    #[test]
    fn test_perception() {
        let fighter = fighter();
        assert_eq!(fighter.perception_bonus(), 4);
    }
}
