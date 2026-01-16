// src/app.rs - Main application struct with Ratatui integration

use std::collections::HashMap;
use std::time::Duration;

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{layout::Rect, Frame};
use tokio::task::JoinHandle;

use crate::components::{
    chat_panel::ChatPanel, command_palette::CommandPalette, confirmation::Confirmation,
    diff_view::DiffView, header::Header, help::HelpOverlay, prompt_input::PromptInput,
    provider_select::ProviderSelect, sidebar::Sidebar, status_bar::StatusBar, Component,
};
use crate::events::{AppEvent, EventHandler};
use crate::executor::{execute_provider_detection, execute_provider_prompt, CommandResult};
use crate::input::keymap::KeymapRegistry;
use crate::input::modes::InputMode;
use crate::input::parser::{KeyParseOutcome, KeySequenceParser};
use crate::message::{Direction, Message};
use crate::model::AppModel;
use crate::state::{
    ChatMessage, DetectionState, ExecutionState, MessageStatus, Mode, ProviderInfo,
};
use crate::ui::layout::{AppLayout, LayoutBreakpoints, LayoutManager};

pub struct App {
    pub model: AppModel,
    event_handler: EventHandler,
    keymap: KeymapRegistry,
    key_parser: KeySequenceParser,
    layout: LayoutManager,
    pending_tasks: HashMap<String, JoinHandle<Result<CommandResult>>>,
    show_splash: bool,
    splash_timer: u8,
    // Components
    header: Header,
    chat: ChatPanel,
    input: PromptInput,
    sidebar: Sidebar,
    diff_view: DiffView,
    provider_select: ProviderSelect,
    confirmation: Confirmation,
    help: HelpOverlay,
    status_bar: StatusBar,
    command_palette: CommandPalette,
}

impl App {
    pub fn new() -> Result<Self> {
        let model = AppModel::new()?;
        Ok(Self {
            model,
            event_handler: EventHandler::new(Duration::from_millis(16)),
            keymap: KeymapRegistry::default_vim(),
            key_parser: KeySequenceParser::new(Duration::from_millis(500)),
            layout: LayoutManager::new(LayoutBreakpoints::default()),
            pending_tasks: HashMap::new(),
            show_splash: true,
            splash_timer: 30,
            header: Header::new(),
            chat: ChatPanel::new(),
            input: PromptInput::new(),
            sidebar: Sidebar::new(),
            diff_view: DiffView::new(),
            provider_select: ProviderSelect::new(),
            confirmation: Confirmation::new(),
            help: HelpOverlay::new(),
            status_bar: StatusBar::new(),
            command_palette: CommandPalette::new(),
        })
    }

    pub async fn run(
        &mut self,
        terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
    ) -> Result<()> {
        // Start provider detection once splash ends
        self.start_provider_detection();

        loop {
            terminal.draw(|f| self.view(f))?;

            if let Some(evt) = self.event_handler.next().await {
                if let Some(msg) = self.handle_event(evt).await? {
                    // Handle OpenEditor specially to access terminal
                    if let Message::OpenEditor { path, line } = msg {
                        self.open_file_in_editor(terminal, path, line).await?;
                    } else {
                        self.handle_message(msg).await?;
                    }
                }
            }

            self.poll_async_tasks().await;

            if self.model.should_quit {
                break;
            }
        }

        Ok(())
    }

    fn view(&mut self, frame: &mut Frame) {
        let area = frame.area();

        if self.show_splash && self.splash_timer > 0 {
            self.splash_timer = self.splash_timer.saturating_sub(1);
            crate::ui::renderers::render_splash(frame, &self.model.theme);
            return;
        } else {
            self.show_splash = false;
        }

        match self.model.state.mode {
            Mode::ProviderSelect => {
                let rect = area;
                self.provider_select.view(frame, rect, &self.model);
            }
            Mode::Confirmation => {
                self.render_main_layout(frame, area);
                let dialog_area = crate::ui::layout::centered_rect_percent(area, 60, 40);
                self.confirmation.view(frame, dialog_area, &self.model);
            }
            Mode::Help => {
                self.render_main_layout(frame, area);
                let dialog_area = crate::ui::layout::centered_rect_percent(area, 80, 80);
                self.help.view(frame, dialog_area, &self.model);
            }
            _ => self.render_main_layout(frame, area),
        }
    }

    fn render_main_layout(&mut self, frame: &mut Frame, area: Rect) {
        let layout = self
            .layout
            .compute(area, self.model.state.sidebar_state.visible);

        match layout {
            AppLayout::Compact {
                header,
                content,
                input,
                status,
            } => {
                self.header.view(frame, header, &self.model);
                self.render_content(frame, content);
                self.render_input(frame, input);
                self.status_bar.view(frame, status, &self.model);
            }
            AppLayout::Normal {
                header,
                content,
                input,
                status,
                sidebar,
            } => {
                self.header.view(frame, header, &self.model);
                if let Some(side) = sidebar {
                    self.render_content(frame, content);
                    self.sidebar.view(frame, side, &self.model);
                } else {
                    self.render_content(frame, content);
                }
                self.render_input(frame, input);
                self.status_bar.view(frame, status, &self.model);
            }
            AppLayout::Wide {
                header,
                chat,
                diff,
                input,
                status,
                sidebar,
            } => {
                self.header.view(frame, header, &self.model);
                self.chat.view(frame, chat, &self.model);
                self.diff_view.view(frame, diff, &self.model);
                self.sidebar.view(frame, sidebar, &self.model);
                self.render_input(frame, input);
                self.status_bar.view(frame, status, &self.model);
            }
        }
    }

    fn render_content(&mut self, frame: &mut Frame, area: Rect) {
        match self.model.state.mode {
            Mode::DiffReview => self.diff_view.view(frame, area, &self.model),
            Mode::Confirmation => {}
            _ => self.chat.view(frame, area, &self.model),
        }
    }

    fn render_input(&mut self, frame: &mut Frame, area: Rect) {
        match self.model.state.mode {
            Mode::CommandMode => self.command_palette.view(frame, area, &self.model),
            Mode::Confirmation | Mode::ProviderSelect => {}
            _ => self.input.view(frame, area, &self.model),
        }
    }

    async fn handle_event(&mut self, evt: AppEvent) -> Result<Option<Message>> {
        match evt {
            AppEvent::Key(key) => self.handle_key(key),
            AppEvent::Resize(w, h) => {
                self.model.state.viewport_cols = w as usize;
                self.model.state.viewport_rows = h as usize;
                Ok(None)
            }
            AppEvent::Tick => Ok(None),
            AppEvent::PromptResult(res) => {
                self.handle_command_result(res);
                Ok(None)
            }
            AppEvent::ProviderDetected(_) => Ok(None),
            AppEvent::Error(e) => {
                eprintln!("event error: {e}");
                Ok(None)
            }
            _ => Ok(None),
        }
    }

    fn handle_key(&mut self, key: KeyEvent) -> Result<Option<Message>> {
        // Global quit
        if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
            self.model.should_quit = true;
            return Ok(None);
        }

        // Let keymap run first
        match self
            .key_parser
            .process(key, &self.keymap, self.model.input_mode)
        {
            KeyParseOutcome::Matched(msg) => return Ok(Some(msg)),
            KeyParseOutcome::Pending => return Ok(None),
            KeyParseOutcome::NoMatch => {}
        }

        // Fallback to focused mode handlers
        match self.model.state.mode {
            Mode::CommandMode => {
                self.handle_command_buffer(key);
                Ok(None)
            }
            _ => {
                self.handle_prompt_input(key)?;
                Ok(None)
            }
        }
    }

    async fn handle_message(&mut self, msg: Message) -> Result<()> {
        match msg {
            Message::Navigate(dir) => self.navigate(dir),
            Message::ScrollTo(idx) => {
                self.model.state.scroll_offset = idx;
            }
            Message::SetMode(mode) => self.model.state.mode = mode,
            Message::SetInputMode(mode) => self.model.input_mode = mode,
            Message::PushInputMode(mode) => self.model.mode_stack.push(mode),
            Message::PopInputMode => {
                if let Some(mode) = self.model.mode_stack.pop() {
                    self.model.input_mode = mode;
                }
            }
            Message::SelectProvider(idx) => {
                if idx < self.model.state.available_providers.len() {
                    let provider_info = &self.model.state.available_providers[idx];
                    let config = self
                        .model
                        .state
                        .config
                        .providers
                        .get(&provider_info.config_key);
                    self.model.state.provider =
                        crate::providers::create_provider(&provider_info.name, config);
                    self.model.state.mode = Mode::PromptEntry;
                }
            }
            Message::DetectProviders => self.start_provider_detection(),
            Message::SubmitPrompt(text) => self.execute_prompt(text),
            Message::CancelPrompt => {
                self.model.state.prompt_buffer.clear();
            }
            Message::AcceptHunk(_) => {}
            Message::RejectHunk(_) => {}
            Message::AcceptAll => {}
            Message::RejectAll => {}
            Message::ApplyChanges => self.model.state.mode = Mode::Confirmation,
            Message::ToggleSidebar => {
                self.model.state.sidebar_state.visible = !self.model.state.sidebar_state.visible
            }
            Message::ToggleHelp => self.model.state.mode = Mode::Help,
            Message::Search(_) => {}
            Message::OpenEditor { .. } => {
                // Handled in run() loop before calling handle_message
            }
            Message::Quit => self.model.should_quit = true,
            Message::Resize(w, h) => {
                self.model.state.viewport_cols = w as usize;
                self.model.state.viewport_rows = h as usize;
            }
            Message::Tick => {}
        }
        Ok(())
    }

    fn navigate(&mut self, dir: Direction) {
        match dir {
            Direction::Down => {
                self.model.state.scroll_offset = self.model.state.scroll_offset.saturating_add(1);
            }
            Direction::Up => {
                self.model.state.scroll_offset = self.model.state.scroll_offset.saturating_sub(1);
            }
            Direction::Left | Direction::Right => {}
        }
    }

    fn handle_command_buffer(&mut self, key: KeyEvent) {
        use crate::input::command_mode::{execute_command, parse_command};
        match key.code {
            KeyCode::Enter => {
                if let Ok(cmd) = parse_command(&self.model.state.command_buffer) {
                    if let Err(e) = execute_command(&cmd, &mut self.model.state) {
                        self.model.state.last_error = Some(crate::error::ErrorDisplay {
                            title: "Command Error".into(),
                            message: e.to_string(),
                            help_url: None,
                        });
                    }
                }
                self.model.state.command_buffer.clear();
                self.model.state.mode = Mode::PromptEntry;
            }
            KeyCode::Esc => {
                self.model.state.command_buffer.clear();
                self.model.state.mode = Mode::PromptEntry;
            }
            KeyCode::Backspace => {
                self.model.state.command_buffer.pop();
            }
            KeyCode::Char(c) => {
                self.model.state.command_buffer.push(c);
            }
            _ => {}
        }
    }

    fn handle_prompt_input(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char(c) => {
                self.model
                    .state
                    .prompt_buffer
                    .insert(self.model.state.cursor_position, c);
                self.model.state.cursor_position += 1;
            }
            KeyCode::Backspace => {
                if self.model.state.cursor_position > 0 {
                    self.model.state.cursor_position -= 1;
                    self.model
                        .state
                        .prompt_buffer
                        .remove(self.model.state.cursor_position);
                }
            }
            KeyCode::Left => {
                if self.model.state.cursor_position > 0 {
                    self.model.state.cursor_position -= 1;
                }
            }
            KeyCode::Right => {
                if self.model.state.cursor_position < self.model.state.prompt_buffer.len() {
                    self.model.state.cursor_position += 1;
                }
            }
            KeyCode::Enter => {
                if !self.model.state.prompt_buffer.is_empty() {
                    let text = std::mem::take(&mut self.model.state.prompt_buffer);
                    self.model.state.cursor_position = 0;
                    self.execute_prompt(text);
                }
            }
            KeyCode::Esc => {
                self.model.state.prompt_buffer.clear();
                self.model.state.cursor_position = 0;
                self.model.state.mode = Mode::ProviderSelect;
            }
            _ => {}
        }
        Ok(())
    }

    /// Open a file in external editor, suspending the TUI
    pub async fn open_file_in_editor(
        &mut self,
        terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
        path: std::path::PathBuf,
        line: Option<usize>,
    ) -> Result<()> {
        crate::ui::editor::open_file_in_editor(terminal, &path, line)?;

        // Reload file changes if it's a pending change
        if self.model.state.pending_changes.contains_key(&path) {
            if let Ok(new_content) = std::fs::read_to_string(&path) {
                // Update pending change with new content
                if let Some(change) = self.model.state.pending_changes.get_mut(&path) {
                    change.proposed_content = new_content;
                }
            }
        }

        Ok(())
    }

    pub fn start_provider_detection(&mut self) {
        self.model.state.available_providers.clear();
        self.model.state.pending_detections.clear();
        self.model.state.detection_state = DetectionState::InProgress;

        let providers_to_check = vec![
            ("claude", "Claude Code", "claude", "claude"),
            ("aider", "Aider", "aider", "aider"),
            ("copilot", "GitHub Copilot CLI", "copilot", "copilot"),
            ("kiro", "Kiro CLI", "kiro", "q"),
        ];

        for (provider_id, display_name, default_cmd, config_key) in providers_to_check {
            if let Some(provider_config) = self.model.state.config.providers.get(config_key) {
                if !provider_config.enabled {
                    continue;
                }

                if let Some(custom_path) = &provider_config.path {
                    self.model.state.available_providers.push(ProviderInfo {
                        name: display_name.to_string(),
                        available: true,
                        cli_command: custom_path.clone(),
                        config_key: config_key.to_string(),
                    });
                    continue;
                }
            }

            let task = tokio::spawn(execute_provider_detection(
                default_cmd,
                provider_id,
                display_name,
                config_key,
            ));

            self.pending_tasks
                .insert(format!("detect_{}", provider_id), task);
            self.model
                .state
                .pending_detections
                .insert(provider_id.to_string());
        }

        for (key, provider_config) in &self.model.state.config.providers {
            if matches!(key.as_str(), "claude" | "aider" | "copilot" | "q" | "kiro") {
                continue;
            }

            if provider_config.enabled {
                if let Some(custom_path) = &provider_config.path {
                    self.model.state.available_providers.push(ProviderInfo {
                        name: provider_config.name.clone().unwrap_or_else(|| key.clone()),
                        available: true,
                        cli_command: custom_path.clone(),
                        config_key: key.clone(),
                    });
                }
            }
        }

        if self.model.state.pending_detections.is_empty() {
            self.model.state.detection_state = DetectionState::Completed;
        }
    }

    pub fn execute_prompt(&mut self, prompt: String) {
        if let Some(provider) = &self.model.state.provider {
            if self.model.state.sessions.current_session_id.is_none() {
                let cwd = std::env::current_dir().unwrap_or_default();
                let _ = self
                    .model
                    .state
                    .sessions
                    .start_session(&provider.name().to_string(), &cwd);
            }

            let user_message = ChatMessage {
                id: self.model.state.chat_history.next_id,
                timestamp: chrono::Utc::now(),
                is_user: true,
                content: prompt.clone(),
                token_count: None,
                cost: None,
                status: MessageStatus::Pending,
                associated_files: vec![],
            };
            self.model.state.chat_history.next_id += 1;
            self.model.state.chat_history.add_message(user_message);
            if let Some(current_id) = self.model.state.sessions.current_session_id.clone() {
                if let Some(session) = self.model.state.sessions.sessions.get_mut(&current_id) {
                    session.messages.push(
                        self.model
                            .state
                            .chat_history
                            .messages
                            .last()
                            .cloned()
                            .unwrap(),
                    );
                }
            }

            self.model.state.status_info.is_working = true;
            self.model.state.status_info.current_task = "Processing prompt...".to_string();
            self.model.state.status_info.start_time = Some(std::time::Instant::now());

            let request = crate::state::PromptRequest {
                prompt,
                context_files: vec![],
                session_id: None,
                working_directory: std::env::current_dir().unwrap_or_default(),
            };

            let args = provider.build_execute_args(&request);
            let cmd = provider.cli_command().to_string();
            let provider_name = provider.name().to_string();

            self.model.state.status_info.provider = provider_name.clone();

            let task =
                tokio::spawn(
                    async move { execute_provider_prompt(&cmd, args, &provider_name).await },
                );

            self.pending_tasks
                .insert("prompt_execution".to_string(), task);
            self.model.state.execution_state = ExecutionState::WaitingForResult;
            self.model.state.mode = Mode::Processing;
        } else {
            self.model.state.last_error = Some(crate::error::ErrorDisplay {
                title: "No Provider".to_string(),
                message: "Please select a provider first".to_string(),
                help_url: None,
            });
            self.model.state.mode = Mode::Error;
        }
    }

    pub async fn poll_async_tasks(&mut self) {
        let mut completed_tasks = Vec::new();

        for (task_id, handle) in &mut self.pending_tasks {
            if handle.is_finished() {
                completed_tasks.push(task_id.clone());
            }
        }

        for task_id in completed_tasks {
            if let Some(handle) = self.pending_tasks.remove(&task_id) {
                match handle.await {
                    Ok(Ok(result)) => {
                        self.handle_command_result(result);
                    }
                    Ok(Err(e)) => {
                        if task_id.starts_with("detect_") {
                            let provider_id = task_id.strip_prefix("detect_").unwrap_or(&task_id);
                            self.model.state.pending_detections.remove(provider_id);

                            let err_msg = e.to_string();
                            if !err_msg.contains("not found") && !err_msg.contains("NotFound") {
                                eprintln!("Provider detection error: {}", e);
                            }

                            if self.model.state.pending_detections.is_empty() {
                                self.model.state.detection_state = DetectionState::Completed;
                            }
                        } else {
                            eprintln!("Command execution error: {}", e);
                        }
                    }
                    Err(e) => {
                        if task_id.starts_with("detect_") {
                            let provider_id = task_id.strip_prefix("detect_").unwrap_or(&task_id);
                            self.model.state.pending_detections.remove(provider_id);

                            if self.model.state.pending_detections.is_empty() {
                                self.model.state.detection_state = DetectionState::Completed;
                            }
                        }
                        eprintln!("Task join error: {}", e);
                    }
                }
            }
        }
    }

    fn handle_command_result(&mut self, result: CommandResult) {
        if let Some(provider_id) = result.context.get("provider_id") {
            if self.model.state.pending_detections.contains(provider_id) {
                self.model.state.pending_detections.remove(provider_id);

                if result.exit_code == Some(0) {
                    if let (Some(display_name), Some(cli_command), Some(config_key)) = (
                        result.context.get("display_name"),
                        result.context.get("cli_command"),
                        result.context.get("config_key"),
                    ) {
                        self.model.state.available_providers.push(ProviderInfo {
                            name: display_name.clone(),
                            available: true,
                            cli_command: cli_command.clone(),
                            config_key: config_key.clone(),
                        });
                    }
                }

                if self.model.state.pending_detections.is_empty() {
                    self.model.state.detection_state = DetectionState::Completed;
                }
            }
        }

        if result.context.get("request_type").map(|s| s.as_str()) == Some("prompt_execution") {
            self.model.state.execution_state = ExecutionState::Idle;
            self.model.state.status_info.is_working = false;

            if let Some(exit_code) = result.exit_code {
                if exit_code == 0 {
                    if let Some(provider) = &self.model.state.provider {
                        let output = String::from_utf8_lossy(&result.stdout);

                        let assistant_message = ChatMessage {
                            id: self.model.state.chat_history.next_id,
                            timestamp: chrono::Utc::now(),
                            is_user: false,
                            content: output.to_string(),
                            token_count: None,
                            cost: None,
                            status: MessageStatus::Success,
                            associated_files: vec![],
                        };
                        self.model.state.chat_history.next_id += 1;
                        self.model.state.chat_history.add_message(assistant_message);
                        if let Some(current_id) =
                            self.model.state.sessions.current_session_id.clone()
                        {
                            if let Some(session) =
                                self.model.state.sessions.sessions.get_mut(&current_id)
                            {
                                if let Some(last) =
                                    self.model.state.chat_history.messages.last().cloned()
                                {
                                    session.messages.push(last);
                                }
                            }
                        }

                        match provider.parse_file_changes(&output) {
                            Ok(changes) => {
                                self.model.state.pending_changes.clear();
                                self.model.state.hunks.clear();
                                self.model.state.overlay_diff_state.proposed_changes.clear();

                                use crate::diff::{extract_hunks, generate_diff};

                                for change in changes {
                                    self.model
                                        .state
                                        .pending_changes
                                        .insert(change.path.clone(), change.clone());

                                    let original = change.original_content.as_deref().unwrap_or("");
                                    let proposed = &change.proposed_content;
                                    let diff = generate_diff(original, proposed);
                                    let hunks = extract_hunks(&change.path, &diff);

                                    let mut line_decorations = Vec::new();
                                    for hunk in &hunks {
                                        for line_change in &hunk.changes {
                                            let decoration_type = match line_change.tag {
                                                crate::state::ChangeTag::Insert => {
                                                    crate::state::DecorationType::Addition
                                                }
                                                crate::state::ChangeTag::Delete => {
                                                    crate::state::DecorationType::Deletion
                                                }
                                                crate::state::ChangeTag::Equal => {
                                                    crate::state::DecorationType::Context
                                                }
                                            };

                                            let line_num = line_change
                                                .new_line_num
                                                .or(line_change.old_line_num)
                                                .unwrap_or(0);

                                            let decoration = crate::state::LineDecoration {
                                                line_number: line_num,
                                                decoration_type,
                                                original_text: if matches!(
                                                    line_change.tag,
                                                    crate::state::ChangeTag::Delete
                                                        | crate::state::ChangeTag::Equal
                                                ) {
                                                    Some(line_change.content.clone())
                                                } else {
                                                    None
                                                },
                                                new_text: if matches!(
                                                    line_change.tag,
                                                    crate::state::ChangeTag::Insert
                                                        | crate::state::ChangeTag::Equal
                                                ) {
                                                    Some(line_change.content.clone())
                                                } else {
                                                    None
                                                },
                                                accepted: None,
                                            };

                                            line_decorations.push(decoration);
                                        }
                                    }

                                    let proposed_change = crate::state::ProposedChange {
                                        id: self
                                            .model
                                            .state
                                            .overlay_diff_state
                                            .proposed_changes
                                            .len(),
                                        file_path: change.path.clone(),
                                        original_content: original.to_string(),
                                        proposed_content: proposed.clone(),
                                        line_decorations,
                                        status: crate::state::ChangeStatus::Pending,
                                    };

                                    self.model
                                        .state
                                        .overlay_diff_state
                                        .proposed_changes
                                        .push(proposed_change);
                                }

                                self.model.state.mode = Mode::DiffReview;
                            }
                            Err(e) => {
                                self.model.state.last_error = Some(crate::error::ErrorDisplay {
                                    title: "Parse Error".to_string(),
                                    message: format!("Failed to parse provider output: {}", e),
                                    help_url: None,
                                });
                                self.model.state.mode = Mode::Error;
                            }
                        }
                    }
                } else {
                    let stderr_str = String::from_utf8_lossy(&result.stderr);

                    let error_message = ChatMessage {
                        id: self.model.state.chat_history.next_id,
                        timestamp: chrono::Utc::now(),
                        is_user: false,
                        content: format!("Error: {}", stderr_str),
                        token_count: None,
                        cost: None,
                        status: MessageStatus::Error,
                        associated_files: vec![],
                    };
                    self.model.state.chat_history.next_id += 1;
                    self.model.state.chat_history.add_message(error_message);

                    self.model.state.last_error = Some(crate::error::ErrorDisplay {
                        title: "Provider Error".to_string(),
                        message: format!("Command failed (exit {}): {}", exit_code, stderr_str),
                        help_url: None,
                    });
                    self.model.state.mode = Mode::Error;
                }
            }
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new().unwrap()
    }
}
