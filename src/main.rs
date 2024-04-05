mod templates;

use std::sync::Arc;

use axum::{
    extract::{Query, State},
    routing::{get, post},
    Router,
};
use maud::Markup;
use tokio::{net::TcpListener, sync::RwLock};

type EditorState = Arc<RwLock<Editor>>;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/keyboard/type", post(keyboard_type))
        .with_state(Arc::new(RwLock::new(Editor::new())));
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root(State(editor): State<EditorState>) -> Markup {
    let editor = editor.read().await;

    templates::page::page(templates::editor::editor_page(&editor))
}

#[derive(serde::Deserialize)]
struct KeyboardTypeParams {
    #[serde(default)]
    is_ctrl: bool,
    #[serde(default)]
    is_shift: bool,
    #[serde(default)]
    is_alt: bool,
    c: char,
}

async fn keyboard_type(
    State(editor): State<EditorState>,
    Query(KeyboardTypeParams {
        is_ctrl,
        is_shift,
        is_alt,
        c,
    }): Query<KeyboardTypeParams>,
) -> Markup {
    let mut editor = editor.write().await;

    if !is_ctrl && !is_shift && !is_alt {
        editor.character_create(c);
    } else if !is_ctrl && is_shift && !is_alt {
        editor.character_create(c.to_ascii_uppercase());
    }

    templates::editor::editor_template(&editor)
}

struct Editor {
    lines: Vec<Vec<char>>,
    cursor: (usize, usize),
}

impl Editor {
    fn new() -> Self {
        Self {
            lines: vec![vec![]],
            cursor: (0, 0),
        }
    }

    fn lines(&self) -> &Vec<Vec<char>> {
        &self.lines
    }

    fn character_create(&mut self, c: char) {
        let (mut line, mut column) = self.cursor;

        if line >= self.lines.len() {
            self.lines.push(Vec::new());
        }

        let current_line_cursor = if self.lines[line].len() < column {
            self.lines[line].len()
        } else {
            column
        };
        self.lines[line].insert(current_line_cursor, c);

        while self.lines[line].len() > 80 {
            let c = self.lines[line].pop().unwrap();
            line += 1;
            if self.lines.len() <= line {
                self.lines.push(Vec::new());
            }
            self.lines[line].insert(0, c);
        }

        column += 1;
        if column >= 80 {
            line += 1;
            column = 0;
        }

        self.cursor = (line, column);
    }
}
