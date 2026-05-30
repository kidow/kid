use gpui::{App, Menu, MenuItem, OsAction};
use release_channel::ReleaseChannel;
use terminal_view::terminal_panel;
use zed_actions::{debug_panel, dev};

pub fn app_menus(cx: &mut App) -> Vec<Menu> {
    use zed_actions::Quit;

    let mut view_items = vec![
        MenuItem::action(
            "확대",
            zed_actions::IncreaseBufferFontSize { persist: false },
        ),
        MenuItem::action(
            "축소",
            zed_actions::DecreaseBufferFontSize { persist: false },
        ),
        MenuItem::action(
            "확대/축소 초기화",
            zed_actions::ResetBufferFontSize { persist: false },
        ),
        MenuItem::action(
            "모든 확대/축소 초기화",
            zed_actions::ResetAllZoom { persist: false },
        ),
        MenuItem::separator(),
        MenuItem::action("왼쪽 독 표시 전환", workspace::ToggleLeftDock),
        MenuItem::action("오른쪽 독 표시 전환", workspace::ToggleRightDock),
        MenuItem::action("아래쪽 독 표시 전환", workspace::ToggleBottomDock),
        MenuItem::action("모든 독 표시 전환", workspace::ToggleAllDocks),
        MenuItem::submenu(Menu {
            name: "편집기 레이아웃".into(),
            disabled: false,
            items: vec![
                MenuItem::action("위로 분할", workspace::SplitUp::default()),
                MenuItem::action("아래로 분할", workspace::SplitDown::default()),
                MenuItem::action("왼쪽으로 분할", workspace::SplitLeft::default()),
                MenuItem::action("오른쪽으로 분할", workspace::SplitRight::default()),
            ],
        }),
        MenuItem::separator(),
        MenuItem::action("프로젝트 패널", zed_actions::project_panel::ToggleFocus),
        MenuItem::action("개요 패널", outline_panel::ToggleFocus),
        MenuItem::action("터미널 패널", terminal_panel::ToggleFocus),
        MenuItem::action("디버거 패널", debug_panel::ToggleFocus),
        MenuItem::separator(),
        MenuItem::action("진단", diagnostics::Deploy),
        MenuItem::separator(),
    ];

    if ReleaseChannel::try_global(cx) == Some(ReleaseChannel::Dev) {
        view_items.push(MenuItem::action(
            "GPUI 인스펙터 표시 전환",
            dev::ToggleInspector,
        ));
        view_items.push(MenuItem::separator());
    }

    vec![
        Menu {
            name: "Kid".into(),
            disabled: false,
            items: vec![
                MenuItem::action("Kid 정보", zed_actions::About),
                MenuItem::action("업데이트 확인", auto_update::Check),
                MenuItem::separator(),
                MenuItem::submenu(Menu::new("설정").items([
                    MenuItem::action("설정 열기", zed_actions::OpenSettings),
                    MenuItem::action("설정 파일 열기", super::OpenSettingsFile),
                    MenuItem::action("프로젝트 설정 열기", zed_actions::OpenProjectSettings),
                    MenuItem::action("프로젝트 설정 파일 열기", super::OpenProjectSettingsFile),
                    MenuItem::action("기본 설정 열기", super::OpenDefaultSettings),
                    MenuItem::separator(),
                    MenuItem::action("키맵 열기", zed_actions::OpenKeymap),
                    MenuItem::action("키맵 파일 열기", zed_actions::OpenKeymapFile),
                    MenuItem::action("기본 키 바인딩 열기", zed_actions::OpenDefaultKeymap),
                    MenuItem::separator(),
                    MenuItem::action(
                        "테마 선택...",
                        zed_actions::theme_selector::Toggle::default(),
                    ),
                    MenuItem::action(
                        "아이콘 테마 선택...",
                        zed_actions::icon_theme_selector::Toggle::default(),
                    ),
                ])),
                MenuItem::separator(),
                #[cfg(target_os = "macos")]
                MenuItem::os_submenu("서비스", gpui::SystemMenuType::Services),
                MenuItem::separator(),
                MenuItem::action("확장", zed_actions::Extensions::default()),
                #[cfg(not(target_os = "windows"))]
                MenuItem::action("CLI 설치", install_cli::InstallCliBinary),
                MenuItem::separator(),
                #[cfg(target_os = "macos")]
                MenuItem::action("Kid 가리기", super::Hide),
                #[cfg(target_os = "macos")]
                MenuItem::action("다른 항목 가리기", super::HideOthers),
                #[cfg(target_os = "macos")]
                MenuItem::action("모두 보기", super::ShowAll),
                MenuItem::separator(),
                MenuItem::action("Kid 종료", Quit),
            ],
        },
        Menu {
            name: "파일".into(),
            disabled: false,
            items: vec![
                MenuItem::action("새로 만들기", workspace::NewFile),
                MenuItem::action("새 윈도우", workspace::NewWindow),
                MenuItem::separator(),
                #[cfg(not(target_os = "macos"))]
                MenuItem::action("파일 열기...", workspace::OpenFiles),
                MenuItem::action(
                    if cfg!(not(target_os = "macos")) {
                        "폴더 열기..."
                    } else {
                        "열기…"
                    },
                    workspace::Open::default(),
                ),
                MenuItem::action(
                    "최근 항목 열기...",
                    zed_actions::OpenRecent {
                        create_new_window: false,
                    },
                ),
                MenuItem::action(
                    "원격 열기...",
                    zed_actions::OpenRemote {
                        create_new_window: false,
                        from_existing_connection: false,
                    },
                ),
                MenuItem::separator(),
                MenuItem::action("프로젝트에 폴더 추가…", workspace::AddFolderToProject),
                MenuItem::separator(),
                MenuItem::action("저장", workspace::Save { save_intent: None }),
                MenuItem::action("다른 이름으로 저장…", workspace::SaveAs),
                MenuItem::action("모두 저장", workspace::SaveAll { save_intent: None }),
                MenuItem::separator(),
                MenuItem::action(
                    "편집기 닫기",
                    workspace::CloseActiveItem {
                        save_intent: None,
                        close_pinned: true,
                    },
                ),
                MenuItem::action("프로젝트 닫기", workspace::CloseProject),
                MenuItem::action("윈도우 닫기", workspace::CloseWindow),
            ],
        },
        Menu {
            name: "편집".into(),
            disabled: false,
            items: vec![
                MenuItem::os_action("실행 취소", editor::actions::Undo, OsAction::Undo),
                MenuItem::os_action("다시 실행", editor::actions::Redo, OsAction::Redo),
                MenuItem::separator(),
                MenuItem::os_action("잘라내기", editor::actions::Cut, OsAction::Cut),
                MenuItem::os_action("복사", editor::actions::Copy, OsAction::Copy),
                MenuItem::action("복사 후 공백 제거", editor::actions::CopyAndTrim),
                MenuItem::os_action("붙여넣기", editor::actions::Paste, OsAction::Paste),
                MenuItem::separator(),
                MenuItem::action("찾기", search::buffer_search::Deploy::find()),
                MenuItem::action("프로젝트에서 찾기", workspace::DeploySearch::default()),
                MenuItem::separator(),
                MenuItem::action(
                    "줄 주석 표시 전환",
                    editor::actions::ToggleComments::default(),
                ),
            ],
        },
        Menu {
            name: "선택".into(),
            disabled: false,
            items: vec![
                MenuItem::os_action(
                    "모두 선택",
                    editor::actions::SelectAll,
                    OsAction::SelectAll,
                ),
                MenuItem::action("선택 영역 확장", editor::actions::SelectLargerSyntaxNode),
                MenuItem::action("선택 영역 축소", editor::actions::SelectSmallerSyntaxNode),
                MenuItem::action("다음 형제 선택", editor::actions::SelectNextSyntaxNode),
                MenuItem::action(
                    "이전 형제 선택",
                    editor::actions::SelectPreviousSyntaxNode,
                ),
                MenuItem::separator(),
                MenuItem::action(
                    "위에 커서 추가",
                    editor::actions::AddSelectionAbove {
                        skip_soft_wrap: true,
                    },
                ),
                MenuItem::action(
                    "아래에 커서 추가",
                    editor::actions::AddSelectionBelow {
                        skip_soft_wrap: true,
                    },
                ),
                MenuItem::action(
                    "다음 항목 선택",
                    editor::actions::SelectNext {
                        replace_newest: false,
                    },
                ),
                MenuItem::action(
                    "이전 항목 선택",
                    editor::actions::SelectPrevious {
                        replace_newest: false,
                    },
                ),
                MenuItem::action("모든 항목 선택", editor::actions::SelectAllMatches),
                MenuItem::separator(),
                MenuItem::action("줄 위로 이동", editor::actions::MoveLineUp),
                MenuItem::action("줄 아래로 이동", editor::actions::MoveLineDown),
                MenuItem::action("선택 영역 복제", editor::actions::DuplicateLineDown),
            ],
        },
        Menu {
            name: "보기".into(),
            disabled: false,
            items: view_items,
        },
        Menu {
            name: "이동".into(),
            disabled: false,
            items: vec![
                MenuItem::action("뒤로", workspace::GoBack),
                MenuItem::action("앞으로", workspace::GoForward),
                MenuItem::separator(),
                MenuItem::action("명령 팔레트...", zed_actions::command_palette::Toggle),
                MenuItem::separator(),
                MenuItem::action("파일로 이동...", workspace::ToggleFileFinder::default()),
                // MenuItem::action("Go to Symbol in Project", project_symbols::Toggle),
                MenuItem::action(
                    "편집기 내 심볼로 이동...",
                    zed_actions::outline::ToggleOutline,
                ),
                MenuItem::action("줄/열로 이동...", editor::actions::ToggleGoToLine),
                MenuItem::separator(),
                MenuItem::action("정의로 이동", editor::actions::GoToDefinition),
                MenuItem::action("선언으로 이동", editor::actions::GoToDeclaration),
                MenuItem::action("타입 정의로 이동", editor::actions::GoToTypeDefinition),
                MenuItem::action(
                    "모든 참조 찾기",
                    editor::actions::FindAllReferences::default(),
                ),
                MenuItem::separator(),
                MenuItem::action("다음 문제", editor::actions::GoToDiagnostic::default()),
                MenuItem::action(
                    "이전 문제",
                    editor::actions::GoToPreviousDiagnostic::default(),
                ),
            ],
        },
        Menu {
            name: "실행".into(),
            disabled: false,
            items: vec![
                MenuItem::action(
                    "작업 실행",
                    zed_actions::Spawn::ViaModal {
                        reveal_target: None,
                    },
                ),
                MenuItem::action("디버거 시작", debugger_ui::Start),
                MenuItem::separator(),
                MenuItem::action("tasks.json 편집...", crate::zed::OpenProjectTasks),
                MenuItem::action("debug.json 편집...", zed_actions::OpenProjectDebugTasks),
                MenuItem::separator(),
                MenuItem::action("계속", debugger_ui::Continue),
                MenuItem::action("스텝 오버", debugger_ui::StepOver),
                MenuItem::action("스텝 인투", debugger_ui::StepInto),
                MenuItem::action("스텝 아웃", debugger_ui::StepOut),
                MenuItem::separator(),
                MenuItem::action("중단점 표시 전환", editor::actions::ToggleBreakpoint),
                MenuItem::action("중단점 편집", editor::actions::EditLogBreakpoint),
                MenuItem::action("모든 중단점 지우기", debugger_ui::ClearAllBreakpoints),
            ],
        },
        Menu {
            name: "윈도우".into(),
            disabled: false,
            items: vec![
                MenuItem::action("최소화", super::Minimize),
                MenuItem::action("확대/축소", super::Zoom),
                MenuItem::separator(),
            ],
        },
    ]
}
