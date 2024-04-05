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

    let (current_line, current_column) = editor.cursor;

    templates::page::page(html! {
        div style="padding: 0.6em; display: flex; flex-direction: column; align-items: center;" {
            div style="height: calc(1em * 26); background-color: #000000; white-space: pre-wrap; font-family: monospace; display: flex;" {
                div style="width: calc(0.6em * 80); display: flex; flex-direction: column;" {
                    @for (line_number, line) in lines.iter().enumerate() {
                        div style=(if line_number == current_line { "background-color: #202020;" } else { "" }) {
                            @for (column_number, c) in line.iter().enumerate() {
                                span style=(
                                    if line_number==current_line && column_number == current_column {
                                        "border-left: 1px solid #eeeeee"
                                    } else if line_number==current_line && column_number == line.len() - 1 && current_column >= line.len() {
                                        "border-right: 1px solid #eeeeee"
                                    } else {
                                        ""
                                    }
                                ) { (c) }
                            }
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
