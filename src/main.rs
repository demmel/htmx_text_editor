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

    templates::page::page(html! {
        div style="padding: 0.6em; display: flex; flex-direction: column; align-items: center;" {
            (templates::editor::editor(&editor))
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
