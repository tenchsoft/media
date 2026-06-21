# Tench-Media 레포 추출 — 구현 계획

## Objective

`~/tench/Tench-One/` 모노레포에서 미디어 관련 4개 앱(view, pixel-design, player, composer)과 14개 크레이트, 관련 도구/문서를 `~/tench-media/`로 추출하여 완전히 독립적인 Cargo 워크스페이스를 구성한다.

---

## 사전 분석 결과

### 경로 참조 불일치 (추출 시 수정 필요)

| 위치 | 현재 참조 방식 | 수정 방향 |
|------|---------------|----------|
| `apps/pixel-design/src-tauri/Cargo.toml:21` | `path = "../../../crates/tench-ui"` | `workspace = true` |
| `apps/player/src-tauri/Cargo.toml:23` | `path = "../../../crates/tench-ui"` | `workspace = true` |
| `apps/composer/src-tauri/Cargo.toml:21` | `path = "../../../crates/tench-ui"` | `workspace = true` |
| `crates/tench-ui-test/Cargo.toml:10` | `path = "../tench-ui"` | `workspace = true` |

### 비워크스페이스 외부 의존성 (인라인 버전)

- `apps/player/src-tauri/Cargo.toml:32-33`: `souvlaki = "0.8"` (플랫폼 조건부)
- `crates/tench-ui/`: 다수 인라인 의존성 (vello, parley, accesskit 등 — optional feature 게이팅)
- `crates/media-playback/`: gstreamer 계열 (optional feature)
- `crates/media-runtime/`: `gif = "0.13"`

---

## Implementation Plan

### Phase 1: 디렉토리 구조 생성

- [x] **1.1** `~/tench-media/` 하위 디렉토리 구조 생성
  - `apps/view/src-tauri/`, `apps/pixel-design/src-tauri/`, `apps/player/src-tauri/`, `apps/composer/src-tauri/`
  - `crates/` 하위 14개 크레이트 디렉토리
  - `tools/architecture-guard/`
  - `plans/` 하위 spec/design/background/implement/test 각 앱별 서브디렉토리
  - `template/`
  - `.gitea/workflows/`
  - 근거: extraction-tench-media.md:109-167에 정의된 디렉토리 구조에 맞춤

### Phase 2: 앱 소스 복사

- [x] **2.1** `~/tench/Tench-One/apps/view/` → `~/tench-media/apps/view/` 전체 복사
- [x] **2.2** `~/tench/Tench-One/apps/pixel-design/` → `~/tench-media/apps/pixel-design/` 전체 복사
- [x] **2.3** `~/tench/Tench-One/apps/player/` → `~/tench-media/apps/player/` 전체 복사
- [x] **2.4** `~/tench/Tench-One/apps/composer/` → `~/tench-media/apps/composer/` 전체 복사
  - 근거: 각 앱은 Tauri 2 앱으로 src-tauri/ 하위에 Cargo.toml, src/, capabilities/, frontend/ 등 포함

### Phase 3: 크레이트 소스 복사

- [x] **3.1** `crates/tench-ui/` 복사 — UI 프레임워크 (vello, parley 기반)
- [x] **3.2** `crates/ui-automation-core/` 복사 — UI 자동화 노드 시스템
- [x] **3.3** `crates/tench-ui-test/` 복사 — 헤드리스 테스트 하네스
- [x] **3.4** `crates/shared-types/` 복사 — 공유 타입 정의
- [x] **3.5** `crates/storage-core/` 복사 — 로컬 스토리지 정책
- [x] **3.6** `crates/fs-core/` 복사 — 로컬 파일/권한
- [x] **3.7** `crates/media-core/` 복사 — 미디어 탐색
- [x] **3.8** `crates/image-core/` 복사 — 이미지 프리미티브
- [x] **3.9** `crates/image-runtime/` 복사 — 이미지 런타임
- [x] **3.10** `crates/pixel-core/` 복사 — 픽셀 레벨 연산
- [x] **3.11** `crates/media-runtime/` 복사 — 미디어 런타임
- [x] **3.12** `crates/media-playback/` 복사 — 미디어 재생 프리미티브
- [x] **3.13** `crates/subtitle-core/` 복사 — 자막 처리
- [x] **3.14** `crates/composer-core/` 복사 — 비디오 편집 코어
  - 근거: extraction-tench-media.md:24-39에 명시된 14개 크레이트

### Phase 4: 도구 및 템플릿 복사

- [x] **4.1** `~/tench/Tench-One/tools/architecture-guard/` → `~/tench-media/tools/architecture-guard/` 복사
  - `line_budget_baseline.txt`을 이 레포 14개 크레이트 기준으로 재생성 필요
- [x] **4.2** `~/tench/Tench-One/template/` → `~/tench-media/template/` 전체 복사 (5개 템플릿 파일)

### Phase 5: Plans 문서 복사

- [x] **5.1** `plans/spec/view/`, `plans/spec/pixel-design/`, `plans/spec/player/`, `plans/spec/composer/` 복사
- [x] **5.2** `plans/design/view/`, `plans/design/pixel-design/`, `plans/design/player/`, `plans/design/composer/` 복사
- [x] **5.3** `plans/background/view/`, `plans/background/pixel-design/`, `plans/background/player/`, `plans/background/composer/` 복사
- [x] **5.4** `plans/implement/view/`, `plans/implement/pixel-design/`, `plans/implement/player/`, `plans/implement/composer/` 복사
- [x] **5.5** `plans/test/view/`, `plans/test/pixel-design/`, `plans/test/player/`, `plans/test/composer/` 복사
- [x] **5.6** `plans/fix/` 하위에서 view/pixel-design/player/composer 관련 파일만 복사
  - 근거: extraction-tench-media.md:219 — 관련 문서만 복사

### Phase 6: 워크스페이스 루트 Cargo.toml 작성

- [x] **6.1** `[workspace]` 섹션 작성 — members 20개 (앱 4 + 크레이트 14 + 도구 1 + 빈칸 1), resolver = "3"
- [x] **6.2** `[workspace.package]` 작성 — version = "0.1.0", edition = "2021", license = "UNLICENSED", authors = ["Tench"]
- [x] **6.3** `[workspace.dependencies]` 정리 — 14개 내부 크레이트 경로 + 외부 의존성만 포함
  - 내부 크레이트: tench-ui, tench-ui-automation-core, tench-ui-test, tench-shared-types, tench-storage-core, tench-fs-core, tench-media-core, tench-image-core, tench-image-runtime, tench-pixel-core, tench-media-runtime, tench-media-playback, tench-subtitle-core, tench-composer-core
  - 외부 의존성 (워크스페이스 참조로 사용되는 것만): tauri, tauri-build, tauri-plugin-dialog, serde, serde_json, image, pollster, base64, kamadak-exif, sevenz-rust2, zip, unrar-ng, getrandom, aes-gcm, sha2
  - 근거: extraction-tench-media.md:173-203

### Phase 7: Cargo.toml 경로 참조 정리

- [x] **7.1** `apps/pixel-design/src-tauri/Cargo.toml` — tench-ui를 `path = "../../../crates/tench-ui"` → `workspace = true`로 변경
- [x] **7.2** `apps/player/src-tauri/Cargo.toml` — tench-ui를 `path = "../../../crates/tench-ui"` → `workspace = true`로 변경
- [x] **7.3** `apps/composer/src-tauri/Cargo.toml` — tench-ui를 `path = "../../../crates/tench-ui"` → `workspace = true`로 변경
- [x] **7.4** `crates/tench-ui-test/Cargo.toml` — tench-ui를 `path = "../tench-ui"` → `workspace = true`로 변경
- [x] **7.5** 모든 앱/크레이트의 `workspace = true` 참조가 새 워크스페이스 루트에서 올바르게 해석되는지 확인
  - 근거: extraction-tench-media.md:215-216 — path 참조 정리

### Phase 8: Cargo.lock 생성

- [x] **8.1** `~/tench-media/`에서 `cargo generate-lockfile` 실행
  - 근거: extraction-tench-media.md:216

### Phase 9: CI 설정

- [x] **9.1** `.gitea/workflows/ci.yml` 작성
  - `cargo check --workspace --locked`
  - `cargo test --workspace --locked`
  - `architecture-guard` 실행
  - gstreamer 의존성 설치 단계 포함 (media-playback의 gstreamer-backend feature용)
  - 근거: extraction-tench-media.md:217

### Phase 10: 문서 작성

- [x] **10.1** `AGENTS.md` 작성 — Tench-Media 전용 에이전트 워크플로우 규칙
- [x] **10.2** `ARCHITECTURE.md` 작성 — Tench-Media 아키텍처 다이어그램 및 크레이트 소유권 맵
  - 근거: extraction-tench-media.md:218

### Phase 11: architecture-guard baseline 재생성

- [x] **11.1** `tools/architecture-guard/line_budget_baseline.txt`를 14개 크레이트 + 4개 앱 기준으로 재생성
  - 근거: extraction-tench-media.md:213 — baseline을 이 레포 크레이트 14개로 재생성

### Phase 12: 빌드 검증

- [x] **12.1** `cargo check --workspace --locked` 통과 확인
- [x] **12.2** `cargo test --workspace --locked` 통과 확인
- [x] **12.3** Gitea CI 파이프라인 녹색 확인
  - 근거: extraction-tench-media.md:221-223

---

## Verification Criteria

- `cargo check --workspace --locked` 성공
- `cargo test --workspace --locked` 성공
- 모든 경로 참조가 `workspace = true`로 통일됨
- `[workspace.dependencies]`에 불필요한 외부 의존성이 없음
- 4개 앱, 14개 크레이트, 1개 도구가 workspace members에 포함됨
- Gitea CI 파이프라인 녹색

---

## Potential Risks and Mitigations

1. **워크스페이스 의존성 누락**
   - 완화: 14개 크레이트의 Cargo.toml을 모두 검사하여 `.workspace = true`로 참조하는 외부 의존성을 빠짐없이 `[workspace.dependencies]`에 포함

2. **tench-ui 인라인 의존성 충돌**
   - 완화: tench-ui는 optional feature 게이팅을 위해 인라인 버전을 사용하는 deps가 많음. 이들은 워크스페이스 deps와 중복되더라도 그대로 유지 (Cargo가 허용하는 패턴)

3. **gstreamer 시스템 라이브러리 의존성**
   - 완화: CI에 gstreamer 설치 단계 포함, `gstreamer-backend` feature는 default가 아니므로 기본 빌드에는 영향 없음

4. **plans/ 문서 필터링 누락**
   - 완화: 복사 후 view/pixel-design/player/composer 외의 앱 관련 문서가 섞여 있는지 검증

5. **architecture-guard baseline 불일치**
   - 완화: 복사 후 baseline을 새 워크스페이스 기준으로 완전히 재생성

---

## Alternative Approaches

1. **스크립트 자동화**: 전체 추출 과정을 셸 스크립트로 작성하여 일괄 실행. 장점: 반복 가능, 단점: 초기 작성 비용
2. **수동 복사 + 점진적 수정**: 파일을 먼저 전부 복사한 후 Cargo.toml을 개별 수정. 장점: 단순, 단점: 누락 위험
3. **cargo workspace inheritance 활용**: 현재 방식과 동일하며, 이미 Tench-One에서 사용 중인 패턴이므로 그대로 적용
