# The HTMX text editor no one asked for.

This project was made for me to try out **axum** + **maud** + **htmx**. It consists of a text editor hosted on a server controlled by AJAX calls from htmx.

## [axum](https://github.com/tokio-rs/axum)

A lightweight web framework built on rust.

## [maud](https://maud.lambda.xyz/)

A macro-based templating library for rust.

## [htmx](https://htmx.org/)

Adds AJAX to all element types and makes swapping some target with the response easy. A hypermedia-based alternative for adding small amonuts of interactivity to an app.

# Run

Don't run this on a public server. The text editor is edittable by anyone without any kind of auth, and I cannot guarantee your safety beyond whatever rust might give you. You've been warned.

```
cargo run
```

Server will be hosted at `0.0.0.0:3000`
