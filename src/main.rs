mod templates;

use std::sync::Arc;

use axum::{extract::State, routing::get, Router};
use maud::{html, Markup};
use tokio::{net::TcpListener, sync::RwLock};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .with_state(Arc::new(RwLock::new(Editor::new())));
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root(State(editor): State<Arc<RwLock<Editor>>>) -> Markup {
    let editor = editor.read().await;
    let lines = editor.lines();

    let line_well_depth = (lines.len() as f32).log10().ceil() as usize + 2; // 2 for padding
    let line_well_style = format!("width: calc(0.6em * {line_well_depth}); padding-right: 0.6em; padding-left: 0.6em; background-color: #333333; text-align: right;");

    let editor_well_depth = line_well_depth + 80 + 3; // 80 for max line length, 3 for padding
    let editor_well_style = format!("width: calc(0.6em * {editor_well_depth}); background-color: #000000; padding: 0.6em; white-space: pre-wrap; font-family: monospace;");

    templates::page::page(html! {
        div style="padding: 0.6em; display: flex; flex-direction: column; align-items: center;" {
            div style=(editor_well_style) {
                @for (line_number, line) in lines.iter().enumerate() {
                    div {
                        span style=(line_well_style) { (line_number + 1) }
                        span style="padding-right: 0.6em" {}
                        @for c in line {
                            span { (c) }
                        }
                    }
                }
            }
        }
    })
}

struct Editor {
    lines: Vec<Vec<char>>,
    cursor: (usize, usize),
}

impl Editor {
    fn new() -> Self {
        Self {
            lines: vec![vec!['H', 'e', 'l', 'l', 'o'], vec!['W', 'o', 'r', 'l', 'd']],
            cursor: (1, 5),
        }
    }

    fn lines(&self) -> &Vec<Vec<char>> {
        &self.lines
    }
}
