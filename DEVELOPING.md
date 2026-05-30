# kid 개발 가이드 (명령어 모음)

모든 명령은 저장소 루트(`/Users/kidow/Documents/dev/kidow/kid`)에서 실행.

## 0. 최초 1회 — 빌드 환경 준비 (macOS)

```sh
brew install cmake                                            # 확장/wasmtime 빌드에 필요
sudo xcode-select -s /Applications/Xcode.app/Contents/Developer
sudo xcodebuild -license accept
xcodebuild -downloadComponent MetalToolchain                 # gpui Metal 셰이더 컴파일용 (macOS 15+)
xcrun metal --version                                         # 버전 출력되면 OK
```

Rust 툴체인은 `rust-toolchain.toml`로 자동 고정됨(별도 설치 불필요, rustup만 있으면 됨).

---

## 1. 그냥 데스크탑 앱 열기 (컴파일 안 함)

이미 빌드된 바이너리를 **직접 실행** → 즉시 창이 뜸. `cargo run`보다 빠름.

```sh
./target/release/zed                       # 빈 창
./target/release/zed ~/path/to/project     # 폴더 열기
```

- debug 바이너리가 있으면: `./target/debug/zed`
- 바이너리가 없으면(아직 한 번도 안 빌드했으면) → 아래 2번으로 먼저 빌드.

---

## 2. 코드 수정 후 — 증분 재빌드

바꾼 crate + 그걸 의존하는 crate만 다시 컴파일됨(전체 재빌드 아님).

**개발 반복은 debug 사용 (빠름):**
```sh
cargo run -p zed                           # 빌드 + 실행 (한 번에)
cargo build -p zed && ./target/debug/zed   # 빌드와 실행 분리
```

**배포/성능 측정만 release (느림, 최적화):**
```sh
cargo run --release -p zed
cargo build --release -p zed && ./target/release/zed
```

언제 오래 걸리나:
- `editor` / `gpui` / `workspace` 같은 **core crate** 수정 → 의존하는 crate 전부 재빌드.
- `browser_panel` 같은 **leaf crate** 수정 → 그 crate만 + `zed` 재링크(254MB 링크라 release는 1–3분).
- **에셋(테마/키맵 JSON)** 수정도 바이너리에 임베드되므로 재빌드 필요.
- core 안 건드린 증분은 보통 수십 초~1분.

> 팁: `cargo run --release`가 매번 느린 건 코드를 안 바꿔도 254MB 바이너리를 재링크하기 때문. 코드 안 바꿨으면 1번(바이너리 직접 실행)을 써라.

---

## 3. 그 외 (빌드만 / 린트 / 테스트)

**빌드만 (실행 안 함):**
```sh
cargo build -p zed                         # debug
cargo build --release -p zed               # release
cargo build --workspace --all-targets      # 전체 crate + 테스트 코드까지 컴파일 확인
```

**린트 (커밋/PR 전 필수 — 경고도 에러로 취급):**
```sh
./script/clippy                            # 전체 워크스페이스
./script/clippy -p zed                     # zed crate만 (빠름)
```

**테스트:**
```sh
cargo test -p zed                          # zed crate
cargo test -p <crate>                      # 특정 crate (예: cargo test -p editor)
cargo test --workspace                     # 전체 (무거움)
```

**포맷 / git:**
```sh
cargo fmt                                  # 코드 포맷
git status
git log --oneline -10
```

---

## 브라우저 패널 사용법

- **토글:** `Cmd+Shift+B` (우측 dock에 열림/포커스)
- **기본 주소:** `http://localhost:3000` → 먼저 dev 서버를 띄워야 내용이 보임(`npm run dev` 등)
- URL 입력창에 주소 입력 + <kbd>Enter</kbd> 로 이동, 새로고침 버튼으로 리로드

## 터미널에서 AI 사용

에디터 내장 AI 없음. 통합 터미널 열고(`` Ctrl+` ``) `claude` 또는 `codex` 실행.

---

## 자주 막히는 것

| 증상 | 원인 / 해결 |
|------|------------|
| `cargo run`이 너무 느림 | 코드 안 바꿨으면 `./target/release/zed` 직접 실행 |
| `unable to find utility "metal"` | Metal 툴체인 미설치 → `xcodebuild -downloadComponent MetalToolchain` |
| `failed to spawn cmake` | `brew install cmake` |
| 빌드는 되는데 앱이 느림 | debug 바이너리임 → 체감 성능 보려면 `--release` |
