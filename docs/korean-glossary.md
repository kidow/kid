# Kid 한국어 인터페이스 용어집 (Korean UI Glossary)

Kid는 영어 UI를 한국어로 직접 치환(in-place)한 개인용 포크다. 이 문서는 모든 번역의 단일 기준(SSOT)이다. 향후 업스트림 pull로 새 영어 문자열이 유입되면 이 용어집을 기준으로 재번역한다.

## 정책

- **톤**: 명사형 간결체 기본. 영어 원본이 명령형/명사("Save", "Close All")이므로 한국어도 짧게.
- **예외 — 다이얼로그/에러/확인 메시지**: 존댓말 서술형. 예) "정말 진행하시겠습니까?", "저장에 실패했습니다."
- **전문용어**: 외래어 유지(한국 개발 현업 표준). 순화 번역(반영/가지 등) 금지.

## 번역 금지 대상 (CRITICAL)

다음은 **절대 번역하지 않는다**. 사용자에게 보이는 UI 텍스트만 번역한다.

- `.action(...)` 의 action 식별자 인자 (예: `editor::Save`, `"workspace::Open"`)
- keymap / 키바인딩 문자열, action 네임스페이스
- 로그 메시지 (`log::`, `error!`, `warn!`, `info!`, `debug!`, `anyhow!` 컨텍스트)
- 테스트 코드 / 픽스처 / 스냅샷 데이터
- 설정 키 이름, JSON 필드명, 직렬화 식별자
- 파일 경로, URL, 명령어 이름, 환경 변수

## 핵심 용어

### 파일/편집
| English | 한국어 |
|---|---|
| File | 파일 |
| Folder | 폴더 |
| Save | 저장 |
| Save All | 모두 저장 |
| Open | 열기 |
| Close | 닫기 |
| Close All | 모두 닫기 |
| New | 새로 만들기 |
| Rename | 이름 변경 |
| Delete | 삭제 |
| Cut | 잘라내기 |
| Copy | 복사 |
| Paste | 붙여넣기 |
| Undo | 실행 취소 |
| Redo | 다시 실행 |
| Find | 찾기 |
| Replace | 바꾸기 |

### Git (외래어)
| English | 한국어 |
|---|---|
| Commit | 커밋 |
| Branch | 브랜치 |
| Stage | 스테이지 |
| Unstage | 스테이지 취소 |
| Push | 푸시 |
| Pull | 풀 |
| Merge | 머지 |
| Stash | 스태시 |
| Diff | 차이 |
| Discard | 변경 취소 |

### 터미널/패널
| English | 한국어 |
|---|---|
| Terminal | 터미널 |
| Project | 프로젝트 |
| Outline | 개요 |
| Search | 검색 |
| Buffer | 버퍼 |
| Tab | 탭 |
| Panel | 패널 |
| Toggle | 표시 전환 |
| Split | 분할 |

### 설정/공통
| English | 한국어 |
|---|---|
| Settings | 설정 |
| Preferences | 환경설정 |
| Enable | 사용 |
| Disable | 사용 안 함 |
| Default | 기본값 |
| Theme | 테마 |
| Keymap | 키맵 |
| Extension | 확장 |
| Reset | 초기화 |

### 다이얼로그/에러 (존댓말 예외)
| English | 한국어 |
|---|---|
| Are you sure? | 정말 진행하시겠습니까? |
| ... failed | ...에 실패했습니다 |
| Saved | 저장됨 |
