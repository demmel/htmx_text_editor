use maud::{html, Markup};

use crate::Editor;

pub fn editor_page(editor: &Editor) -> Markup {
    html! {
        style {
            r#"
            .current_line {
                background-color: #202020;
            }

            .current_cursor {
                animation: blinker 1s linear infinite;
                border-left: 1px solid #ffffff;
            }

            @keyframes blinker {
                50% {
                    opacity: 0;
                }
            }
            "#
        }
        div style="padding: 1em; display: flex; flex-direction: column; align-items: center;" {
            h1 {"The HTMX text editor no one asked for."}
            div style="margin-top: 1em; height: calc(1em * 26); background-color: #000000; white-space: pre-wrap; font-family: monospace; display: flex;" {
                div style="width: calc(0.552em * 80); display: flex; flex-direction: column;" {
                    (editor_template(editor))
                }
            }
            (keyboard())
            a href="https://github.com/demmel/htmx_text_editor" style="margin-top: 1em;"{
                "Github"
            }
        }
    }
}

pub fn editor_template(editor: &Editor) -> Markup {
    let lines = editor.lines();

    let (current_line, current_column) = editor.cursor;

    let lines_to_display = 26;
    let start_line = current_line.saturating_sub(lines_to_display / 2);
    let end_line = (start_line + lines_to_display).min(lines.len());
    let start_line = end_line.saturating_sub(lines_to_display);
    html! {
        div .editor {
            @for (line_number, l) in lines.iter().enumerate().skip(start_line).take(lines_to_display) {
                (line(l, line_number, current_line, current_column))
            }
        }
    }
}

fn line(line: &[char], line_number: usize, current_line: usize, current_column: usize) -> Markup {
    html! {
        div .current_line[line_number == current_line] {
            @for (column_number, c) in line.iter().enumerate() {
                (character(*c, line_number == current_line && column_number == current_column))
            }
            (if line_number == current_line && current_column >= line.len() {
                html!{span .current_cursor {}}
            } else {
                html!{}
            })
        }
    }
}

fn character(c: char, is_cursor: bool) -> Markup {
    html! {
        span .current_cursor[is_cursor] { (c) }
    }
}

fn keyboard() -> Markup {
    html! {
        @for c in '!'..='~' {
            @for is_ctrl in [false, true] {
                @for is_shift in [false, true] {
                    @for is_alt in [false, true] {
                        (key(c, is_ctrl, is_shift, is_alt))
                    }
                }
            }
        }
    }
}

fn key(c: char, is_ctrl: bool, is_shift: bool, is_alt: bool) -> Markup {
    let ctrl_key = format!("{}ctrlKey", if is_ctrl { "" } else { "!" });
    let shift_key = format!("{}shiftKey", if is_shift { "" } else { "!" });
    let alt_key = format!("{}altKey", if is_alt { "" } else { "!" });

    html! {
        div
            style="display: none"
            hx-target=".editor"
            hx-swap="outerHTML"
            hx-post=(format!("/keyboard/type?c={c}&&is_ctrl={is_ctrl}&&is_shift={is_shift}&&is_alt={is_alt}"))
            hx-trigger=(format!("keydown[{ctrl_key}&&{shift_key}&&{alt_key}&&key=='{c}'] from:body")) {}
    }
}
