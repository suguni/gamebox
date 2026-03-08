이 프로젝트는 게임을 만들면서 Rust 와 Bevy 를 학습하기 위한 곳이다.

## Claude 역할

- 코드를 직접 작성하기보다, 요청자가 원하는 기능을 개발하는데 필요한 정보들을 상세히 알려주고 좋은 코드가 만들어지도록 가이드한다.
- 상세하다는 것은 코드를 왜 그렇게 작성해야 하는지, 동작하는 원리가 어떻게 되는지를 알려준다는 것을 의미한다.
- Rust 및 사용 라이브러리는 항상 최신버전을 기준으로 알려준다.

## 프로젝트 구조

Cargo workspace 구성:
- `ya2048/` - 2048 게임 (Bevy UI 적용 중, 브랜치: feature/setup_bevy)
- `pong/` - Pong 게임 (Bevy)
- `life_game/` - Conway's Game of Life (브랜치: life_game)

workspace 공통 의존성은 루트 `Cargo.toml`의 `[workspace.dependencies]`에서 관리한다.

## 개발 방침

- 도메인 로직과 UI(Bevy)를 분리한다.
- 도메인 로직은 TDD로 먼저 구현한다.
- 각 게임은 별도 브랜치에서 개발하고 완성 후 main에 머지한다.

## life_game 개발 현황

브랜치: `life_game`

도메인 로직 (`life_game/src/main.rs`):
- `Grid` 구조체: `width`, `height`, `cells: Vec<bool>` (1차원 플랫 배열)
- `Grid::new(width, height)` - 모든 셀이 죽은 상태로 초기화
- `Grid::from_str(s)` - `#`=살아있음, `.`=죽음, 줄바꿈으로 행 구분
- `live_neighbor_count(x, y)` - 8방향 이웃 수 계산, 경계 처리 포함
- `next_generation()` - Conway 규칙 적용해 새 Grid 반환 (순수 함수)

다음 단계: `lib.rs`로 도메인 분리 후 Bevy UI 연결
