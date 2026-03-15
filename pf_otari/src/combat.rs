use crate::dice::{check_degree, DegreeOfSuccess};

struct CombatTurn {
    actions: u8,
    reaction: bool,
}

impl CombatTurn {
    fn new() -> Self {
        Self {
            actions: 3,
            reaction: true,
        }
    }

    fn remaining_actions(&self) -> u8 {
        self.actions
    }

    fn spend_action(&mut self, cost: u8) -> Result<(), ()> {
        if cost > self.actions {
            Err(())
        } else {
            self.actions -= cost;
            Ok(())
        }
    }

    fn has_reaction(&self) -> bool {
        self.reaction
    }

    fn spend_reaction(&mut self) -> Result<(), ()> {
        if !self.reaction {
            Err(())
        } else {
            self.reaction = false;
            Ok(())
        }
    }
}

/// 공격명중굴림 판정
///
/// # Arguments
/// `roll` - 굴림
/// `attack_bonus` - 공격 보너스
/// `target_ac` - 대상 방어 수치 (armor class)
fn strike(roll: u8, attack_bonus: i32, target_ac: i32) -> DegreeOfSuccess {
    let roll = roll as i32;
    check_degree(roll + attack_bonus, target_ac, roll)
}

fn calc_damage(degree: DegreeOfSuccess, dice_roll: i32, bonus: i32) -> i32 {
    let p = match degree {
        DegreeOfSuccess::CriticalSuccess => 2,
        DegreeOfSuccess::Success => 1,
        DegreeOfSuccess::Failure | DegreeOfSuccess::CriticalFailure => 0,
    };

    if p > 0 {
        i32::max(1, (dice_roll + bonus) * p)
    } else {
        0
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_new_turn_has_3_actions() {
        let turn = CombatTurn::new();
        assert_eq!(turn.remaining_actions(), 3);
    }

    #[test]
    fn test_spend_action() {
        let mut turn = CombatTurn::new();

        let action_result = turn.spend_action(1);

        assert!(action_result.is_ok());
        assert_eq!(turn.remaining_actions(), 2);
    }

    #[test]
    fn test_spend_action_insufficient() {
        let mut turn = CombatTurn::new();

        assert!(turn.spend_action(2).is_ok());

        assert_eq!(turn.remaining_actions(), 1);

        let action_result = turn.spend_action(2);
        assert!(action_result.is_err());
    }

    #[test]
    fn test_has_reaction() {
        let turn = CombatTurn::new();

        assert!(turn.has_reaction());
    }

    #[test]
    fn test_spend_reaction() {
        let mut turn = CombatTurn::new();

        let reaction_result = turn.spend_reaction();
        assert!(reaction_result.is_ok());

        let reaction_result = turn.spend_reaction();
        assert!(reaction_result.is_err());
    }

    use super::DegreeOfSuccess::*;

    #[test]
    fn test_strike() {
        // roll = 20
        assert_eq!(strike(20, 0, 20), CriticalSuccess);

        // roll + attack_bonus >= target_ac -> Success
        assert_eq!(strike(10, 2, 11), Success);

        // roll + attack_bonus < target_ac -> Failure
        assert_eq!(strike(10, 2, 13), Failure);

        // roll + attack_bonus >= target_ac + 10 -> CriticalSuccess
        assert_eq!(strike(10, 10, 10), CriticalSuccess);

        // roll = 1 -> 강등
        assert_eq!(strike(1, 15, 10), Failure);

        // 정확히 AC와 동일 (경계)
        assert_eq!(strike(10, 0, 10), Success);

        // AC보다 1 부족 (경계)
        assert_eq!(strike(10, 0, 11), Failure);

        // 차이 -10 (경계)
        assert_eq!(strike(10, 0, 20), CriticalFailure);

        // 차이 -10 초과
        assert_eq!(strike(10, 0, 21), CriticalFailure);

        // 차이 +9, CriticalSuccess 아님 (경계)
        assert_eq!(strike(10, 9, 10), Success);

        // roll=20, 기본 CriticalFailure → ascend → Failure
        assert_eq!(strike(20, 0, 30), Failure);

        // roll=1, 기본 CriticalSuccess → descend → Success
        assert_eq!(strike(1, 20, 10), Success);

        // roll=1, 기본 Failure → descend → CriticalFailure
        assert_eq!(strike(1, 0, 10), CriticalFailure);
    }

    #[test]
    fn test_calc_damage() {
        // failure, critical failure 이면 0
        assert_eq!(calc_damage(Failure, 10, 0), 0);
        assert_eq!(calc_damage(CriticalFailure, 10, 0), 0);

        assert_eq!(calc_damage(Success, 4, 2), 6);
        assert_eq!(calc_damage(CriticalSuccess, 4, 2), 12);

        // bonus가 0일 때 (보너스 없는 무기)
        assert_eq!(calc_damage(Success, 6, 0), 6);
        assert_eq!(calc_damage(CriticalSuccess, 6, 0), 12);

        // bonus가 음수일 때 (패널티 상황)
        assert_eq!(calc_damage(Success, 6, -2), 4);
        assert_eq!(calc_damage(CriticalSuccess, 6, -2), 8);

        // dice_roll 최솟값 1, bonus 포함
        assert_eq!(calc_damage(Success, 1, 4), 5); // 1d6 최솟값
        assert_eq!(calc_damage(CriticalSuccess, 1, 4), 10);

        // bonus 음수, 데미지 최솟값 1 보장
        assert_eq!(calc_damage(Success, 1, -5), 1);
        assert_eq!(calc_damage(CriticalSuccess, 1, -5), 1);

        // Failure는 음수여도 0
        assert_eq!(calc_damage(Failure, 1, -5), 0);
    }
}
