use maud::{html, Markup};

use crate::Editor;

pub fn editor(editor: &Editor) -> Markup {
    let lines = editor.lines();

    let (current_line, current_column) = editor.cursor;

    html! {
        style {
            r#"
            .cursor {
                border-left: 1px solid #ffffff;
            }
            "#
        }
        div style="height: calc(1em * 26); background-color: #000000; white-space: pre-wrap; font-family: monospace; display: flex;" {
            div style="width: calc(0.6em * 80); display: flex; flex-direction: column;" {
                @for (line_number, l) in lines.iter().enumerate() {
                    (line(l, line_number, current_line, current_column))
                }
            }
        }
    }
}

fn line(line: &[char], line_number: usize, current_line: usize, current_column: usize) -> Markup {
    html! {
        div style=(if line_number == current_line { "background-color: #202020;" } else { "" }) {
            @for (column_number, c) in line.iter().enumerate() {
                (character(*c, line_number == current_line && column_number == current_column))
            }
            (if line_number == current_line && current_column >= line.len() {
                html!{span .cursor {}}
            } else {
                html!{}
            })
        }
    }
}

fn character(c: char, is_cursor: bool) -> Markup {
    html! {
        span .cursor[is_cursor] { (c) }
    }
}
