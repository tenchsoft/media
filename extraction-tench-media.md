# Tench-Media 레포 추출 계획

## 원칙

- 완전히 독립적인 Cargo 워크스페이스
- 필요한 공유 크레이트는 물리적으로 복사
- 다른 레포와의 동기화 없음. 각 크레이트는 독립적으로 진화

---

## 앱

| 앱 | 패키지명 | 비고 |
|----|---------|------|
| view | `tench-view` | 이미지 뷰어 |
| pixel-design | `tench-pixel-design` | 이미지 에디터 |
| player | `tench-player` | 미디어 플레이어 |
| composer | `tench-composer` | 비디오 에디터 |

---

## 포함할 크레이트

| 크레이트 | 패키지명 | 직접 소비 앱 | 내부 의존성 |
|----------|---------|-------------|------------|
| tench-ui | `tench-ui` | view, pixel-design, player, composer | `ui-automation-core` |
| ui-automation-core | `tench-ui-automation-core` | 전체 (dev) | 없음 |
| tench-ui-test | `tench-ui-test` | 전체 (dev) | `tench-ui`, `ui-automation-core` |
| shared-types | `tench-shared-types` | view, player | 없음 |
| storage-core | `tench-storage-core` | view | 없음 |
| media-core | `tench-media-core` | (image-core, media-runtime 경유) | `fs-core` |
| fs-core | `tench-fs-core` | (media-core 경유) | 없음 |
| image-core | `tench-image-core` | view | `media-core` |
| image-runtime | `tench-image-runtime` | view, pixel-design | `image-core`, `pixel-core`, `storage-core`, `tench-ui` |
| pixel-core | `tench-pixel-core` | pixel-design, (image-runtime 경유) | 없음 |
| media-runtime | `tench-media-runtime` | player, composer | `media-core`, `shared-types`, `storage-core`, `subtitle-core` |
| media-playback | `tench-media-playback` | player | 없음 |
| subtitle-core | `tench-subtitle-core` | (media-runtime 경유) | 없음 |
| composer-core | `tench-composer-core` | composer | 없음 |

---

## 크레이트 의존성 그래프

```
tench-ui ──────── ui-automation-core
tench-ui-test ─── tench-ui, ui-automation-core

media-core ────── fs-core
image-core ────── media-core
image-runtime ─── image-core, pixel-core, storage-core, tench-ui
pixel-core (독립)

media-runtime ─── media-core, shared-types, storage-core, subtitle-core
media-playback (독립)
subtitle-core (독립)
composer-core (독립)
```

---

## 앱별 상세 의존성

### view (`apps/view/src-tauri`)

```
tench-image-core
tench-storage-core
tench-image-runtime
tench-ui (features = ["tauri"])
tench-shared-types
tench-ui-test (dev)
tench-ui-automation-core (dev)
```

### pixel-design (`apps/pixel-design/src-tauri`)

```
tench-image-runtime
tench-pixel-core
tench-ui (features = ["tauri"])
tench-ui-test (dev)
tench-ui-automation-core (dev)
```

### player (`apps/player/src-tauri`)

```
tench-media-playback
tench-media-runtime
tench-ui (features = ["tauri"])
tench-shared-types
tench-ui-test (dev)
tench-ui-automation-core (dev)
```

### composer (`apps/composer/src-tauri`)

```
tench-composer-core
tench-media-runtime
tench-ui (features = ["tauri"])
tench-ui-test (dev)
tench-ui-automation-core (dev)
```

---

## 디렉토리 구조

```
Tench-Media/
├── Cargo.toml              (워크스페이스 루트)
├── Cargo.lock
├── .gitea/
│   └── workflows/ci.yml
├── AGENTS.md
├── ARCHITECTURE.md
├── apps/
│   ├── view/
│   │   └── src-tauri/
│   ├── pixel-design/
│   │   └── src-tauri/
│   ├── player/
│   │   └── src-tauri/
│   └── composer/
│       └── src-tauri/
├── crates/
│   ├── tench-ui/
│   ├── ui-automation-core/
│   ├── tench-ui-test/
│   ├── shared-types/
│   ├── storage-core/
│   ├── fs-core/
│   ├── media-core/
│   ├── image-core/
│   ├── image-runtime/
│   ├── pixel-core/
│   ├── media-runtime/
│   ├── media-playback/
│   ├── subtitle-core/
│   └── composer-core/
├── plans/
│   ├── spec/view/
│   ├── spec/pixel-design/
│   ├── spec/player/
│   ├── spec/composer/
│   ├── design/view/
│   ├── design/pixel-design/
│   ├── design/player/
│   ├── design/composer/
│   ├── background/view/
│   ├── background/pixel-design/
│   ├── background/player/
│   ├── background/composer/
│   ├── implement/view/
│   ├── implement/pixel-design/
│   ├── implement/player/
│   ├── implement/composer/
│   ├── test/view/
│   ├── test/pixel-design/
│   ├── test/player/
│   └── test/composer/
├── template/
└── tools/
    └── architecture-guard/
```

---

## 워크스페이스 설정

```toml
[workspace]
members = [
  "apps/view/src-tauri",
  "apps/pixel-design/src-tauri",
  "apps/player/src-tauri",
  "apps/composer/src-tauri",
  "crates/tench-ui",
  "crates/ui-automation-core",
  "crates/tench-ui-test",
  "crates/shared-types",
  "crates/storage-core",
  "crates/fs-core",
  "crates/media-core",
  "crates/image-core",
  "crates/image-runtime",
  "crates/pixel-core",
  "crates/media-runtime",
  "crates/media-playback",
  "crates/subtitle-core",
  "crates/composer-core",
  "tools/architecture-guard",
]
resolver = "3"

[workspace.package]
version = "0.1.0"
edition = "2021"
license = "UNLICENSED"
authors = ["Tench"]
```

---

## 이관 체크리스트

1. Gitea에 `Tench-Media` 빈 레포 생성
2. `apps/view`, `apps/pixel-design`, `apps/player`, `apps/composer` 복사
3. 14개 크레이트를 `crates/` 하위에 복사
4. `tools/architecture-guard` 복사, baseline을 이 레포 크레이트 14개로 재생성
5. 워크스페이스 루트 `Cargo.toml` 작성 (위 설정 기준)
6. `[workspace.dependencies]` 정리 — 이 레포에서 사용하는 외부 의존성만 남기기
7. 각 앱/크레이트의 `path` 참조 정리 — `path = "../../../crates/..."` → `path = "crates/..."` 로 통일
8. `cargo generate-lockfile` 실행
9. `.gitea/workflows/ci.yml` 작성
10. `AGENTS.md`, `ARCHITECTURE.md` 작성
11. `plans/` 하위에서 view/pixel-design/player/composer 관련 문서만 복사
12. `template/` 복사
13. `cargo check --workspace --locked` 통과 확인
14. `cargo test --workspace --locked` 통과 확인
15. Gitea CI 파이프라인 녹색 확인
