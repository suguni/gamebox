# PF2e 규칙 정리

## 주사위 굴림 기본 개념

### 판정의 구조

```
d20 + 보너스 합계  vs  DC (Difficulty Class)
```

거의 모든 판정은 이 한 줄로 요약된다.

---

### d20의 특별한 역할

주사위 눈 자체(natural roll)가 중요하다:
- **자연 20**: 성공 등급을 한 단계 올림 (숫자 +1이 아니라 등급 승격)
- **자연 1**: 성공 등급을 한 단계 내림 (숫자 -1이 아니라 등급 강등)

등급 순서: `CriticalFailure → Failure → Success → CriticalSuccess`

예) DC 20, 총합 12 (원래 Failure):
- 자연 20 → Success로 승격
- 자연 1 → CriticalFailure로 강등

이미 최고/최저 등급이면 변화 없음.

---

### 성공 등급 4단계

| 결과 | 조건 |
|------|------|
| Critical Success | DC + 10 이상 |
| Success | DC 이상 |
| Failure | DC 미만 |
| Critical Failure | DC - 10 이하 |

4단계가 중요한 이유: 단순 성공/실패가 아니라 **얼마나 잘했는지**가 효과에 영향을 준다.
예) 공격 Critical Success → 피해 2배 / 마법 저항 Critical Success → 피해 없음

---

### 보너스의 종류

- **능력치 수정값**: `(점수 - 10) / 2` (STR/DEX/CON/INT/WIS/CHA)
- **숙련도 보너스**: Untrained(0) / Trained(레벨+2) / Expert(레벨+4) / Master(레벨+6) / Legendary(레벨+8)
- **상황 보너스**: 유리한 위치, 아이템 등

---

### 주요 판정 종류

- **공격 굴림**: `d20 + 공격 보너스` vs 상대 AC
- **내성 굴림(Saving Throw)**: `d20 + 내성 보너스` vs 마법 DC
- **기술 판정**: `d20 + 기술 보너스` vs 난이도 DC

---

## 전투 턴 구조

### 3액션 시스템

한 캐릭터의 턴은 다음으로 구성된다:

- **3 액션**: 자유롭게 소비 (공격, 이동, 주문 등)
- **1 리액션**: 턴 밖에서 조건부로 사용 (기회 공격 등)
- **자유 행동**: 액션 소비 없음 (말하기 등)

---

### Strike (공격)

Strike는 1액션을 소비하는 기본 근접/원거리 공격이다.

**명중 굴림:**
```
d20 + 공격 보너스 vs 대상 AC
```

**데미지 계산:**

| 결과 | 데미지 |
|------|--------|
| Critical Success | `(주사위 + 보너스) × 2` |
| Success | `주사위 + 보너스` |
| Failure | 0 |
| Critical Failure | 0 |

- 보너스는 힘 수정치, 아이템 보너스, 상태 보너스 등의 합산
- 명중 시 데미지 최솟값은 **1** (패널티로 0 이하가 되어도 1)

---

### Multiple Attack Penalty (MAP)

같은 턴에 Strike를 여러 번 할수록 명중률이 떨어진다:

| 이번 턴 몇 번째 Strike | 일반 무기 | Agile 무기 |
|---|---|---|
| 1번째 | 0 | 0 |
| 2번째 | -5 | -4 |
| 3번째 이상 | -10 | -8 |

예시:
```
턴 시작: strike_count = 0
첫 번째 Strike → map = 0,  strike_count = 1
두 번째 Strike → map = -5, strike_count = 2
세 번째 Strike → map = -10, strike_count = 3
```
