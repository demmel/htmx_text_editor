use maud::{html, Markup, PreEscaped};

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
            script type="text/javascript" {
                (PreEscaped(r#"
                    htmx.on("keydown", function(event) {
                        const is_ctrl = !!event.ctrlKey;
                        const is_shift = !!event.shiftKey;
                        const is_alt = !!event.altKey;
                        const key = event.key;

                        if (key.length == 1) {
                            htmx.ajax(
                                "POST", 
                                `/keyboard/type?key=${key}&is_ctrl=${is_ctrl}&is_shift=${is_shift}&is_alt=${is_alt}`, 
                                {
                                    target: ".editor",
                                    swap: "outerHTML",
                                }
                            );
                            event.preventDefault();
                        } else {}
                    });
                "#))
            }
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
