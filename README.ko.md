[English](./README.md) | [한국어](./README.ko.md)

# kid

[Zed](https://github.com/zed-industries/zed)의 가벼운 1인용 포크입니다. 빠른 로컬 편집에 집중하며,
**터미널 기반 AI**(Claude Code, Codex CLI)와 내장된 **브라우저 사이드 패널**을 제공합니다.

kid는 Zed의 편집기 내부 AI, 협업 기능, 텔레메트리 스택을 덜어내어 더 가벼운 편집기 코어로 만들고,
로컬 앱을 코드 옆에서 바로 미리 볼 수 있는 네이티브 브라우저 패널을 추가합니다.

## 상위 Zed와의 차이점

**제거된 기능**

- **편집기 내 AI / LLM** - 에이전트 패널, 인라인 편집 예측, Copilot, 그리고 모든 모델
  제공자(Anthropic, OpenAI, Google, Ollama, Bedrock, Mistral, …). 대신 통합 터미널에서
  `claude` 또는 `codex`를 실행하세요.
- **협업 / 멀티플레이어** - 채널, 통화, 화면 공유, 협업 패널, LiveKit 클라이언트,
  협업 서버.
- **텔레메트리** - 사용량 분석 이벤트 수집 전체(어떠한 사용 데이터도 수집하거나 전송하지 않음).
- **Vim/Helix 모달 편집**, 편집기 내 **확장 프로그램 브라우저 UI**, 벤치마크 크레이트.

**유지된 기능** - 편집기, 언어 및 LSP 지원, 프로젝트/파일 관리, Git 및 GitHub 통합,
터미널, 작업, 디버거, **확장 호스트**(언어 서버와 확장 프로그램은 계속 로드됨),
파일 찾기, REPL.

**추가된 기능**

- **브라우저 사이드 패널** - 편집기 옆에 도킹되는 네이티브 WebView([wry](https://github.com/tauri-apps/wry) 기반).
  [브라우저 패널](#브라우저-패널)을 참고하세요.

**기본값** - **Ayu Dark** 테마와 **VS Code** 기본 키맵만 포함됩니다.

## 빌드 방법(macOS, Apple Silicon)

사전 요구 사항:

- **Rust** - 저장소는 `rust-toolchain.toml`로 툴체인을 고정합니다.
- **CMake** - `brew install cmake`(wasmtime/확장 런타임 빌드에 필요).
- **전체 Xcode + Metal 툴체인** - Command Line Tools만으로는 *부족*합니다. gpui가
  빌드 시점에 Metal 셰이더를 컴파일하기 때문입니다:

  ```sh
  sudo xcode-select -s /Applications/Xcode.app/Contents/Developer
  sudo xcodebuild -license accept
  # macOS 15+/Xcode 16.3+에서는 Metal 컴파일러가 별도 구성요소입니다.
  xcodebuild -downloadComponent MetalToolchain
  xcrun metal --version   # 버전이 출력되어야 합니다.
  ```

빌드 및 실행:

```sh
cargo run --release -p zed
```

## 브라우저 패널

- **토글:** `Cmd+Shift+B`로 오른쪽 도크에서 Browser 패널을 열거나 포커스합니다.
- **기본 URL:** `http://localhost:3000`. 바에 URL을 입력하고 <kbd>Enter</kbd>를 누르면
  이동합니다. 새로고침 버튼을 누르면 다시 불러옵니다.
- 이 패널은 **네이티브 WebView**를 호스팅하며, 패널 범위에 맞춰 위치하고 패널이 닫히면 숨겨지므로
  편집기 자체 렌더링과 분리되어 있습니다.

## 라이선스 및 저작자 표시

kid는 **[zed-industries/zed](https://github.com/zed-industries/zed)**의 포크이며,
상위 저장소에서 Apache-2.0 구성요소로 표시된 부분을 제외하고 **GPL-3.0-or-later**로 배포됩니다.
원본 라이선스 텍스트는 [LICENSE-GPL](./LICENSE-GPL)과
[LICENSE-APACHE](./LICENSE-APACHE)에 그대로 보존되어 있습니다. GPL에 따라 완전한 대응 소스는
이 저장소 전체입니다. 편집기의 기반이 된 작업은 Zed Industries와 Zed 기여자들에게 있습니다.
