# Implementation Plan: Info Panel EXIF

## Feature ID
view/info-panel-exif

## Spec Reference
`plans/spec/view/info-panel-exif.md`

## Design Reference
`plans/design/view/info-panel-exif.md`

## Implementation Strategy

### Locate
- File: `apps/view/src-tauri/src/ui/panels.rs`
- State: `apps/view/src-tauri/src/ui/state.rs` — `ViewState::exif_data`

### Change Recipe
1. 정보 패널에서 EXIF 탭 렌더링
2. 카메라, 렌즈, 조리개, 셔터스피드, ISO, GPS 등 메타데이터 표시
3. 이미지 로드 시 EXIF 파싱

### Find Strategies
- `grep -rn "exif" apps/view/src-tauri/src/ui/state.rs`

### Debug ID
`info-panel-exif`

### Verification
- cargo check -p view
