# kid 개발 가이드 (명령어 모음)

모든 명령은 저장소 루트(`/Users/kidow/Documents/dev/kidow/kid`)에서 실행.

## 0. 최초 1회 — 빌드 환경 준비 (macOS)

```sh
brew install cmake lld                                        # cmake: wasmtime 빌드 / lld: 빠른 링크 (.cargo/config.toml에서 사용)
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

## 1-A. 응용 프로그램으로 등록 (Kid.app)

`/Applications/Kid.app`을 만들어 Launchpad / Spotlight / Dock에서 실행:

```sh
script/install-mac-app.sh                # release 빌드 + Kid.app 설치
SKIP_BUILD=1 script/install-mac-app.sh   # 이미 빌드돼 있으면 (재빌드 생략)
```

- Spotlight에서 "Kid" 검색 / Launchpad / Applications에서 더블클릭으로 실행.
- **첫 실행**: 서명 안 된 dev 빌드라 Gatekeeper 경고 → Kid.app 우클릭 > 열기 (1회만).
- 번들 ID `dev.kid.Kid` (공식 Zed.app `dev.zed.Zed`와 분리) → Dock/Launchpad 충돌 없음.
- release 코드 바꾼 뒤 갱신: 스크립트 다시 실행 (또는 `cp target/release/zed /Applications/Kid.app/Contents/MacOS/zed`).
- ⚠️ `APP_NAME`이 아직 "Zed"라 공식 Zed.app과 데이터 폴더·단일 인스턴스 소켓 공유 → **둘 다 동시에 켜지 말 것**. (zed→kid 이름변경 완료하면 해소.)

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

빌드 속도 (lld 링커 + debug 기준, `.cargo/config.toml`에 설정됨):
- `browser_panel` / `title_bar` 같은 **leaf crate** 수정 → 그 crate만 컴파일 + `zed` 재링크 ≈ **~10초** (lld 덕분에 254MB 링크가 빠름).
- `editor` / `gpui` / `workspace` 같은 **core crate** 수정 → 의존 crate 전부 재컴파일 → 수 분 (불가피).
- **에셋(테마/키맵 JSON)** 수정도 바이너리에 임베드되므로 재빌드 필요.
- 전체 재컴파일(수 분)은 첫 빌드 또는 `.cargo/config.toml`·toolchain 변경 시에만.

> 팁: `--release`는 최적화라 컴파일·링크 둘 다 더 느림. UI 반복 수정은 무조건 debug(`cargo run -p zed`). 코드 안 바꿨으면 1번(바이너리 직접 실행).

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
| `cargo run`이 너무 느림 | `--release` 말고 `cargo run -p zed`(debug). 코드 안 바꿨으면 `./target/release/zed` 또는 `./target/debug/zed` 직접 실행 |
| 링크 에러 `ld64.lld ... No such file` 등 | lld 미설치 → `brew install lld` (`.cargo/config.toml`이 lld 경로 참조) |
| `unable to find utility "metal"` | Metal 툴체인 미설치 → `xcodebuild -downloadComponent MetalToolchain` |
| `failed to spawn cmake` | `brew install cmake` |
| 빌드는 되는데 앱이 느림 | debug 바이너리임 → 체감 성능 보려면 `--release` |
