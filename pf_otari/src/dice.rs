#[derive(Debug, PartialEq, Eq)]
pub enum DegreeOfSuccess {
    CriticalSuccess,
    Success,
    Failure,
    CriticalFailure,
}

impl DegreeOfSuccess {
    fn ascend(&self) -> Self {
        match self {
            DegreeOfSuccess::CriticalSuccess => Self::CriticalSuccess,
            DegreeOfSuccess::Success => Self::CriticalSuccess,
            DegreeOfSuccess::Failure => Self::Success,
            DegreeOfSuccess::CriticalFailure => Self::Failure,
        }
    }

    fn descend(&self) -> Self {
        match self {
            DegreeOfSuccess::CriticalSuccess => Self::Success,
            DegreeOfSuccess::Success => Self::Failure,
            DegreeOfSuccess::Failure => Self::CriticalFailure,
            DegreeOfSuccess::CriticalFailure => Self::CriticalFailure,
        }
    }
}

/// d20 판정 결과 계산
///
/// # Arguments
/// * `total` - d20 굴림 + 보너스 합계
/// * `dc` - 난이도 (difficulty class)
/// * `natural_roll` - d20 굴림값, 보너스 미포함
pub fn check_degree(total: i32, dc: i32, natural_roll: i32) -> DegreeOfSuccess {
    let diff = total - dc;

    let success = if diff >= 10 {
        DegreeOfSuccess::CriticalSuccess
    } else if diff >= 0 && diff < 10 {
        DegreeOfSuccess::Success
    } else if diff > -10 && diff < 0 {
        DegreeOfSuccess::Failure
    } else {
        DegreeOfSuccess::CriticalFailure
    };

    if natural_roll == 20 {
        success.ascend()
    } else if natural_roll == 1 {
        success.descend()
    } else {
        success
    }
}

#[cfg(test)]
mod tests {

    use super::DegreeOfSuccess::*;
    use super::*;

    #[test]
    fn test_check_degree() {
        // 기본 판정
        assert_eq!(check_degree(25, 15, 10), CriticalSuccess); // 10 이상 초과
        assert_eq!(check_degree(24, 15, 9), Success); // 9 초과
        assert_eq!(check_degree(15, 15, 10), Success); // DC와 동일
        assert_eq!(check_degree(14, 15, 9), Failure); // 1 미달
        assert_eq!(check_degree(5, 15, 10), CriticalFailure); // 10 이상 미달
        assert_eq!(check_degree(6, 15, 10), Failure); // (딱 9 미달)

        // 자연 20 (한 단계 상승):
        assert_eq!(check_degree(20, 25, 20), Success);
        assert_eq!(check_degree(10, 25, 20), Failure);
        assert_eq!(check_degree(30, 25, 20), CriticalSuccess);

        // 자연 1 (한 단계 하강):
        assert_eq!(check_degree(15, 15, 1), Failure);
        assert_eq!(check_degree(25, 15, 1), Success);
        assert_eq!(check_degree(5, 15, 1), CriticalFailure);
    }
}
