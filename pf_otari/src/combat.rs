use crate::dice::{check_degree, DegreeOfSuccess};

struct CombatTurn {
    actions: u8,
    reaction: bool,
    strike_count: u8,
}

impl CombatTurn {
    fn new() -> Self {
        Self {
            actions: 3,
            reaction: true,
            strike_count: 0,
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

    fn map(&self) -> i32 {
        self.strike_count as i32 * -5
    }

    fn strike_count_up(&mut self) -> Result<(), ()> {
        if self.strike_count == 2 {
            Err(())
        } else {
            self.strike_count += 1;
            Ok(())
        }
    }
}

/// Strike Action
///
/// `combat_turn` CombatTurn
/// `attack_roll` 명중 굴림
/// `attack_bonus` 공격 보너스
/// `target_ac` 대상 AC
/// `damage_roll` 데미지 굴림
/// `damage_bonus` 데미지 보너스
fn strike_action(
    combat_turn: &mut CombatTurn,
    attack_roll: u8,
    attack_bonus: i32,
    target_ac: i32,
    damage_roll: i32,
    damage_bonus: i32,
) -> Result<i32, ()> {
    combat_turn.spend_action(1)?;

    let map = combat_turn.map();
    let _ = combat_turn.strike_count_up();

    let attack_degree = attack(attack_roll, attack_bonus + map, target_ac);
    let damage_value = damage(attack_degree, damage_roll, damage_bonus);
    Ok(damage_value)
}

/// 명중 판정
///
/// # Arguments
/// `roll` - 명중 굴림
/// `attack_bonus` - 공격 보너스
/// `target_ac` - 대상 방어 수치 (armor class)
fn attack(roll: u8, bonus: i32, target_ac: i32) -> DegreeOfSuccess {
    let roll = roll as i32;
    check_degree(roll + bonus, target_ac, roll)
}

/// 데미지 계산
///
/// `degree` - 명중 굴림 판정
/// `roll` - 데미지 굴림
/// `bonus` - 데미지 보너스
fn damage(attack_degree: DegreeOfSuccess, roll: i32, bonus: i32) -> i32 {
    let p = match attack_degree {
        DegreeOfSuccess::CriticalSuccess => 2,
        DegreeOfSuccess::Success => 1,
        DegreeOfSuccess::Failure | DegreeOfSuccess::CriticalFailure => 0,
    };

    if p > 0 {
        i32::max(1, (roll + bonus) * p)
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
        assert_eq!(attack(20, 0, 20), CriticalSuccess);

        // roll + attack_bonus >= target_ac -> Success
        assert_eq!(attack(10, 2, 11), Success);

        // roll + attack_bonus < target_ac -> Failure
        assert_eq!(attack(10, 2, 13), Failure);

        // roll + attack_bonus >= target_ac + 10 -> CriticalSuccess
        assert_eq!(attack(10, 10, 10), CriticalSuccess);

        // roll = 1 -> 강등
        assert_eq!(attack(1, 15, 10), Failure);

        // 정확히 AC와 동일 (경계)
        assert_eq!(attack(10, 0, 10), Success);

        // AC보다 1 부족 (경계)
        assert_eq!(attack(10, 0, 11), Failure);

        // 차이 -10 (경계)
        assert_eq!(attack(10, 0, 20), CriticalFailure);

        // 차이 -10 초과
        assert_eq!(attack(10, 0, 21), CriticalFailure);

        // 차이 +9, CriticalSuccess 아님 (경계)
        assert_eq!(attack(10, 9, 10), Success);

        // roll=20, 기본 CriticalFailure → ascend → Failure
        assert_eq!(attack(20, 0, 30), Failure);

        // roll=1, 기본 CriticalSuccess → descend → Success
        assert_eq!(attack(1, 20, 10), Success);

        // roll=1, 기본 Failure → descend → CriticalFailure
        assert_eq!(attack(1, 0, 10), CriticalFailure);
    }

    #[test]
    fn test_calc_damage() {
        // failure, critical failure 이면 0
        assert_eq!(damage(Failure, 10, 0), 0);
        assert_eq!(damage(CriticalFailure, 10, 0), 0);

        assert_eq!(damage(Success, 4, 2), 6);
        assert_eq!(damage(CriticalSuccess, 4, 2), 12);

        // bonus가 0일 때 (보너스 없는 무기)
        assert_eq!(damage(Success, 6, 0), 6);
        assert_eq!(damage(CriticalSuccess, 6, 0), 12);

        // bonus가 음수일 때 (패널티 상황)
        assert_eq!(damage(Success, 6, -2), 4);
        assert_eq!(damage(CriticalSuccess, 6, -2), 8);

        // dice_roll 최솟값 1, bonus 포함
        assert_eq!(damage(Success, 1, 4), 5); // 1d6 최솟값
        assert_eq!(damage(CriticalSuccess, 1, 4), 10);

        // bonus 음수, 데미지 최솟값 1 보장
        assert_eq!(damage(Success, 1, -5), 1);
        assert_eq!(damage(CriticalSuccess, 1, -5), 1);

        // Failure는 음수여도 0
        assert_eq!(damage(Failure, 1, -5), 0);
    }

    #[test]
    fn test_do_strike() {
        // 1. Strike 시 액션 1 소비
        let mut turn = CombatTurn::new();
        let _ = strike_action(&mut turn, 10, 5, 10, 4, 2);
        assert_eq!(turn.remaining_actions(), 2);

        // 2. 액션 부족 시 Err 반환
        let mut turn = CombatTurn::new();
        let _ = turn.spend_action(3);
        assert!(strike_action(&mut turn, 10, 5, 10, 4, 2).is_err());

        // 3. 명중(Success) 시 Ok(데미지) 반환
        let mut turn = CombatTurn::new();
        assert_eq!(strike_action(&mut turn, 10, 0, 10, 4, 2), Ok(6));

        // 4. 치명타(CriticalSuccess) 시 데미지 2배
        let mut turn = CombatTurn::new();
        assert_eq!(strike_action(&mut turn, 10, 10, 10, 4, 2), Ok(12));

        // 5. 빗나감(Failure) 시 Ok(0) 반환
        let mut turn = CombatTurn::new();
        assert_eq!(strike_action(&mut turn, 5, 0, 20, 4, 2), Ok(0));

        // 6. MAP 적용: 두 번째 Strike에 -5 적용한 결과
        let mut turn = CombatTurn::new();
        let _ = strike_action(&mut turn, 10, 5, 15, 4, 2); // 첫 번째, attack_bonus=5
        assert_eq!(strike_action(&mut turn, 10, 5, 15, 4, 2), Ok(0)); // 두 번째, MAP -5 적용 → 10+0=10 < 15 → Failure
    }

    #[test]
    fn test_map_penalty() {
        let mut turn = CombatTurn::new();
        assert_eq!(turn.map(), 0); // 첫 번째 strike 전

        let _ = turn.strike_count_up();
        assert_eq!(turn.map(), -5); // 두 번째

        let _ = turn.strike_count_up();
        assert_eq!(turn.map(), -10); // 세 번째 이상

        let _ = turn.strike_count_up();
        assert_eq!(turn.map(), -10); // 그 이상도 -10
    }
}
