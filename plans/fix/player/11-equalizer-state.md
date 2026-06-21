# Player: Equalizer 항목 상태 검증 누락

## 문제

`player.equalizer.band.0.plus`와 `player.equalizer.band.4.minus`를 클릭하지만 `state.eq_bands[0]`, `state.eq_bands[4]` 값 변화를 검증하지 않는다. `player.equalizer.preset.bass_boost`도 마찬가지.

## 관련 파일

- **소스**: `apps/player/src-tauri/src/ui/app.rs:1090-1106` (`ClickAction::SetEqBand`, `ClickAction::SetEqPreset`)
- **소스**: `apps/player/src-tauri/src/ui/state.rs:600-601` (eq_bands, eq_preset_idx)
- **테스트**: `apps/player/src-tauri/tests/plan_ui_e2e.rs:343-345`

## 메인 코드 수정

필요 없음. `state.eq_bands`와 `state.eq_preset_idx`가 public.

## E2E 라이브러리 수정

필요 없음.

## 테스트 수정

`apps/player/src-tauri/tests/plan_ui_e2e.rs`:

```rust
// 기존 (라인 343-345):
click(&mut harness, "player.equalizer.band.0.plus");
click(&mut harness, "player.equalizer.band.4.minus");
click(&mut harness, "player.equalizer.preset.bass_boost");

// 수정:
let before_band0 = state(&mut harness).eq_bands[0];
click(&mut harness, "player.equalizer.band.0.plus");
assert!(state(&mut harness).eq_bands[0] > before_band0);

let before_band4 = state(&mut harness).eq_bands[4];
click(&mut harness, "player.equalizer.band.4.minus");
assert!(state(&mut harness).eq_bands[4] < before_band4);

click(&mut harness, "player.equalizer.preset.bass_boost");
let s = state(&mut harness);
assert_eq!(s.eq_bands, [6.0, 4.0, 0.0, 0.0, 0.0]); // Bass Boost preset 값
```
