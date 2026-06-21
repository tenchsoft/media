# Player: Subtitle Style 항목 상태 검증 누락

## 문제

14개 subtitle style 버튼 (font_size, font_family, text_color, bg_opacity, position, stroke_width, shadow_offset 각각 minus/plus)을 클릭하지만 `state.subtitle_style` 필드 변화를 검증하지 않는다.

## 관련 파일

- **소스**: `apps/player/src-tauri/src/ui/app.rs:1019-1021` (ToggleSubtitleStyle)
- **소스**: `apps/player/src-tauri/src/ui/state.rs:290-297` (SubtitleStyle fields: font_size, font_family, text_color, bg_opacity, stroke_width, shadow_offset, position)
- **테스트**: `apps/player/src-tauri/tests/plan_ui_e2e.rs:289-306`

## 메인 코드 수정

subtitle style 버튼들의 ClickAction 핸들러가 `state.subtitle_style` 필드를 수정하는지 확인 필요. 현재 `player.subtitle_style.font_size.minus` 등의 debug_id가 automation_nodes에 등록되어 있는지, 그리고 클릭 시 실제로 state가 변경되는지 확인.

만약 style 버튼들이 ClickAction enum에 정의되어 있지 않다면:
- `ClickAction::AdjustSubtitleStyle(field, delta)` 등 추가 필요
- 각 버튼 클릭 시 `state.subtitle_style`의 해당 필드를 +/- 조정

## E2E 라이브러리 수정

필요 없음.

## 테스트 수정

`apps/player/src-tauri/tests/plan_ui_e2e.rs`:

```rust
// 기존 (라인 289-306): 클릭만 반복
// 수정:
let before_size = state(&mut harness).subtitle_style.font_size;
click(&mut harness, "player.subtitle_style.font_size.minus");
assert!(state(&mut harness).subtitle_style.font_size < before_size);
click(&mut harness, "player.subtitle_style.font_size.plus");
assert_eq!(state(&mut harness).subtitle_style.font_size, before_size);

// 동일 패턴으로 font_family, text_color, bg_opacity, position, stroke_width, shadow_offset 검증
```
