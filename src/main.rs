use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use duckdb::Connection;
use ratatui::{
    Frame, Terminal,
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
};
use std::{env, io};

#[derive(PartialEq)]
enum InputMode {
    Normal,
    Editing,
}

#[derive(PartialEq)]
enum ActiveBlock {
    Logs,
    Detail,
}

struct App {
    conn: Connection,
    items: Vec<String>,
    state: ListState,
    input: String,
    cursor_position: usize,
    input_mode: InputMode,
    active_block: ActiveBlock,
    detail_scroll: u16,
    error_message: Option<String>,
    page_size: usize,
    current_page: usize,
    total_count: usize,
    sort_desc: bool,
    available_columns: Vec<String>,
    sort_col_index: usize,
}

impl App {
    fn new(file_path: &str) -> Result<App, Box<dyn std::error::Error>> {
        let conn = Connection::open_in_memory()?;
        let import_query = format!(
            "CREATE TABLE raw_logs AS SELECT row_number() OVER() as log_id, * FROM read_json_auto('{}')",
            file_path.replace("\\", "/")
        );
        conn.execute(&import_query, [])?;

        let mut stmt = conn.prepare("DESCRIBE raw_logs")?;
        let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
        let available_columns = rows.collect::<Result<Vec<_>, _>>()?;

        let mut app = App {
            conn,
            items: Vec::new(),
            state: ListState::default(),
            input: String::new(),
            cursor_position: 0,
            input_mode: InputMode::Normal,
            active_block: ActiveBlock::Logs,
            detail_scroll: 0,
            error_message: None,
            page_size: 100,
            total_count: 0,
            current_page: 0,
            sort_desc: true,
            available_columns,
            sort_col_index: 0,
        };
        app.run_query();
        Ok(app)
    }

    pub fn run_query(&mut self) {
        let where_clause = if self.input.trim().is_empty() {
            "1=1".to_string()
        } else {
            self.input.clone()
        };

        // 1. カウントの更新
        let count_sql = format!("SELECT COUNT(*) FROM raw_logs WHERE {}", where_clause);
        match self
            .conn
            .query_row(&count_sql, [], |row| row.get::<_, usize>(0))
        {
            Ok(count) => self.total_count = count,
            Err(e) => {
                self.error_message = Some(format!("SQL Error: {}", e));
                return;
            }
        }

        // 2. データの取得
        let sort_col = &self.available_columns[self.sort_col_index];
        let order = if self.sort_desc { "ASC" } else { "DESC" };
        let offset = self.current_page * self.page_size;

        let sql = format!(
            "SELECT log_id, to_json(raw_logs)::TEXT FROM raw_logs WHERE {} ORDER BY \"{}\" {} LIMIT {} OFFSET {}",
            where_clause, sort_col, order, self.page_size, offset
        );

        let query_res = self.conn.prepare(&sql).and_then(|mut stmt| {
            let iter = stmt.query_map([], |row| {
                let id: i64 = row.get(0)?;
                let json_raw: String = row.get(1)?;
                let processed_json =
                    if let Ok(mut v) = serde_json::from_str::<serde_json::Value>(&json_raw) {
                        if let Some(obj) = v.as_object_mut() {
                            obj.remove("log_id");
                        }
                        v.to_string()
                    } else {
                        json_raw
                    };
                Ok(format!("{}\t{}", id, processed_json))
            })?;
            iter.collect::<Result<Vec<String>, _>>()
        });

        match query_res {
            Ok(new_items) => {
                self.items = new_items;
                self.error_message = None;
                if !self.items.is_empty() && self.state.selected().is_none() {
                    self.state.select(Some(0));
                }
            }
            Err(e) => {
                self.error_message = Some(format!("Fetch Error: {}", e));
                self.items.clear();
            }
        }
    }

    // 最大ページ数を計算するメソッド
    fn total_pages(&self) -> usize {
        if self.total_count == 0 {
            1
        } else {
            self.total_count.div_ceil(self.page_size)
        }
    }

    // 次のページへ
    fn next_page(&mut self) {
        let total = self.total_pages();
        self.current_page = (self.current_page + 1) % total;
        self.run_query(); // ページ変更後にクエリを再実行
    }

    // 前のページへ
    fn previous_page(&mut self) {
        let total = self.total_pages();
        self.current_page = (self.current_page + total - 1) % total;
        self.run_query(); // ページ変更後にクエリを再実行
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        println!(
            r#"LogBee: JSON Log Viewer

USAGE:
    logbee <json_file>

FLAGS:
    -h, --help      Prints help information

KEYBINDINGS (Normal Mode):
    i               Edit Query (SQL WHERE clause)
    Enter           Focus Detail View / List View
    j / k           Move selection / Scroll detail
    n / p           Next / Previous page
    s               Toggle Sort Direction (ASC/DESC)
    Tab             Cycle Sort Column
    q               Quit app

QUERY EXAMPLES:
    level >= 40     (Filter for WARN or higher)
    msg LIKE '%err%' (Search messages containing 'err')
"#
        );
        if args.len() < 2 {
            std::process::exit(1);
        } else {
            std::process::exit(0);
        }
    }
    let app = App::new(&args[1])?;
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;
    let _res = run_loop(&mut terminal, app);
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

fn run_loop<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
) -> Result<(), Box<dyn std::error::Error>>
where
    B::Error: std::error::Error + 'static,
{
    loop {
        terminal.draw(|f| ui(f, &mut app))?;
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            if app.input_mode == InputMode::Editing {
                match key.code {
                    KeyCode::Esc => app.input_mode = InputMode::Normal,
                    KeyCode::Left => app.cursor_position = app.cursor_position.saturating_sub(1),
                    KeyCode::Right => {
                        if app.cursor_position < app.input.len() {
                            app.cursor_position += 1;
                        }
                    }
                    KeyCode::Char(c) => {
                        app.input.insert(app.cursor_position, c);
                        app.cursor_position += 1;
                    }
                    KeyCode::Backspace => {
                        if app.cursor_position > 0 {
                            app.input.remove(app.cursor_position - 1);
                            app.cursor_position -= 1;
                        }
                    }
                    KeyCode::Enter => {
                        app.current_page = 0;
                        app.run_query();
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                }
            } else {
                match app.active_block {
                    ActiveBlock::Logs => match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('i') => app.input_mode = InputMode::Editing,
                        KeyCode::Enter => {
                            app.active_block = ActiveBlock::Detail;
                            app.detail_scroll = 0;
                        }
                        KeyCode::Char('n') => {
                            app.next_page();
                            app.detail_scroll = 0;
                        }
                        KeyCode::Char('p') => {
                            app.previous_page();
                            app.detail_scroll = 0;
                        }
                        KeyCode::Char('s') => {
                            app.sort_desc = !app.sort_desc;
                            app.run_query();
                            app.detail_scroll = 0;
                        }
                        KeyCode::Tab => {
                            app.sort_col_index =
                                (app.sort_col_index + 1) % app.available_columns.len();
                            app.run_query();
                            app.detail_scroll = 0;
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            let i = match app.state.selected() {
                                Some(i) => {
                                    if i >= app.items.len().saturating_sub(1) {
                                        0
                                    } else {
                                        i + 1
                                    }
                                }
                                None => 0,
                            };
                            app.state.select(Some(i));
                            app.detail_scroll = 0;
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            let i = match app.state.selected() {
                                Some(i) => {
                                    if i == 0 {
                                        app.items.len().saturating_sub(1)
                                    } else {
                                        i - 1
                                    }
                                }
                                None => 0,
                            };
                            app.state.select(Some(i));
                            app.detail_scroll = 0;
                        }
                        _ => {}
                    },
                    ActiveBlock::Detail => match key.code {
                        KeyCode::Esc | KeyCode::Left | KeyCode::Char('h') => {
                            app.active_block = ActiveBlock::Logs;
                            app.detail_scroll = 0;
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            app.detail_scroll = app.detail_scroll.saturating_add(1)
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            app.detail_scroll = app.detail_scroll.saturating_sub(1)
                        }
                        _ => {}
                    },
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(f.area());

    // 1. Query Area
    let sort_col = &app.available_columns[app.sort_col_index];
    let sort_icon = if app.sort_desc { "▲" } else { "▼" };
    let input_area_title = if let Some(err) = &app.error_message {
        format!(" ❌ Error: {} ", err)
    } else {
        format!(
            " Query [Sort: {} {}] [Page: {} / {}] (Total: {}) ",
            sort_col,
            sort_icon,
            app.current_page + 1,
            app.total_pages(),
            app.total_count
        )
    };

    f.render_widget(
        Paragraph::new(app.input.as_str())
            .style(if app.input_mode == InputMode::Editing {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            })
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(input_area_title),
            ),
        chunks[0],
    );

    if app.input_mode == InputMode::Editing {
        f.set_cursor_position((
            chunks[0].x + 1 + app.cursor_position as u16,
            chunks[0].y + 1,
        ));
    }

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(chunks[1]);

    // 2. Logs Area
    let list_items: Vec<ListItem> = app
        .items
        .iter()
        .map(|line| ListItem::new(line.replace('\t', " | ")))
        .collect();
    let list_block = Block::default()
        .title(" Logs ")
        .borders(Borders::ALL)
        .border_style(if app.active_block == ActiveBlock::Logs {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default()
        });

    f.render_stateful_widget(
        List::new(list_items)
            .block(list_block)
            .highlight_style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("▶ "),
        body_chunks[0],
        &mut app.state,
    );

    // 3. Detail Area
    let detail_text = if let Some(idx) = app.state.selected() {
        if let Some(line) = app.items.get(idx) {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 2 {
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(parts[1]) {
                    serde_json::to_string_pretty(&v)
                        .unwrap_or_default()
                        .replace("\\n", "\n")
                } else {
                    parts[1].to_string()
                }
            } else {
                "".to_string()
            }
        } else {
            "".to_string()
        }
    } else {
        "No selection".to_string()
    };

    let detail_block = Block::default()
        .title(" Detail (j/k: scroll, Esc: back) ")
        .borders(Borders::ALL)
        .border_style(if app.active_block == ActiveBlock::Detail {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });

    f.render_widget(
        Paragraph::new(detail_text)
            .block(detail_block)
            .wrap(Wrap { trim: false })
            .scroll((app.detail_scroll, 0)),
        body_chunks[1],
    );

    // 4. Help Guide
    let help = if app.input_mode == InputMode::Editing {
        " <ENTER>: Search | <ESC>: Cancel "
    } else if app.active_block == ActiveBlock::Logs {
        " <Enter>: Detail | <i>: Edit | <n/p>: Page | <s>: Sort Dir | <Tab>: Sort Key | <q>: Quit "
    } else {
        " <Esc/h>: Back to Logs | <j/k>: Scroll "
    };
    f.render_widget(
        Paragraph::new(help).style(Style::default().bg(Color::DarkGray)),
        chunks[2],
    );
}
