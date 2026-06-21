# Player: Subtitle Drawer 항목 상태 검증 누락

## 문제

subtitle drawer의 offset, builtin track, encoding 항목을 클릭하지만 상태 변화를 검증하지 않는다.

- `player.subtitle.external.0.offset_minus` / `offset_plus` -> `state.subtitle_tracks[0].offset_ms` 변화 미검증
- `player.subtitle.builtin.none` / `player.subtitle.builtin.0` -> `state.active_builtin_subtitle_track` 변화 미검증
- `player.subtitle.encoding.cp1252` -> `state.subtitle_encoding` 변화 미검증

## 관련 파일

- **소스**: `apps/player/src-tauri/src/ui/app.rs:748-753` (SelectSubtitleTrack), `apps/player/src-tauri/src/ui/app.rs:924-940` (SelectBuiltinSubtitleTrack), `apps/player/src-tauri/src/ui/app.rs:1043-1057` (SetSubtitleEncoding)
- **소스**: `apps/player/src-tauri/src/ui/state.rs:396-398` (SubtitleTrack fields), `apps/player/src-tauri/src/ui/state.rs:557` (active_builtin_subtitle_track), `apps/player/src-tauri/src/ui/state.rs:589` (subtitle_encoding)
- **테스트**: `apps/player/src-tauri/tests/plan_ui_e2e.rs:246-250`

## 메인 코드 수정

필요 없음. 관련 state 필드가 모두 public.

## E2E 라이브러리 수정

필요 없음.

## 테스트 수정

`apps/player/src-tauri/tests/plan_ui_e2e.rs`:

```rust
// 기존 (라인 246-250):
click(&mut harness, "player.subtitle.external.0.offset_minus");
click(&mut harness, "player.subtitle.external.0.offset_plus");
click(&mut harness, "player.subtitle.builtin.none");
click(&mut harness, "player.subtitle.builtin.0");
click(&mut harness, "player.subtitle.encoding.cp1252");

// 수정:
let before_offset = state(&mut harness).subtitle_tracks[0].offset_ms;
click(&mut harness, "player.subtitle.external.0.offset_minus");
assert_eq!(state(&mut harness).subtitle_tracks[0].offset_ms, before_offset - 100);

click(&mut harness, "player.subtitle.external.0.offset_plus");
assert_eq!(state(&mut harness).subtitle_tracks[0].offset_ms, before_offset);

click(&mut harness, "player.subtitle.builtin.none");
assert_eq!(state(&mut harness).active_builtin_subtitle_track, -1);

click(&mut harness, "player.subtitle.builtin.0");
assert_eq!(state(&mut harness).active_builtin_subtitle_track, 0);

click(&mut harness, "player.subtitle.encoding.cp1252");
assert_eq!(state(&mut harness).subtitle_encoding, SubtitleEncoding::Cp1252);
```
