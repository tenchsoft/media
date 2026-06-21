# Composer: Export (Native Dialog No-op)

## 문제

`composer.toolbar.export` 버튼 클릭 시 `ClickAction::Export` -> export dialog -> `app_handle` 필요. 하지만 `composer.deliver.export` 경로로 render queue가 열리는 것은 검증되고 있음.

## 관련 파일

- **소스**: `apps/composer/src-tauri/src/ui/mod.rs:239-241`
- **테스트**: `apps/composer/src-tauri/tests/plan_ui_e2e.rs:250-254`

## 메인 코드 수정

`ClickAction::Export`가 render queue를 열고 render job을 생성하는 것은 이미 동작함. 실제 파일 저장은 no-op이지만 UI 상태 변화는 검증됨. 추가 수정 필요 없음.

## E2E 라이브러리 수정

해당 없음.

## 테스트 수정

현재 검증으로 충분:
- `queued.assert_selector_present(&selector("composer.render_job.pause"))`
- `queued.assert_selector_present(&selector("composer.render_job.cancel"))`
- `queued.assert_selector_present(&selector("composer.render_queue.close"))`

추가 검증:
```rust
// render job이 생성되었는지 state 검증
assert!(!state(&mut harness).render_jobs.is_empty());
```
