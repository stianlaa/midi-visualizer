### Getting started

Like most rust applications, this is built using `cargo build` and run using `cargo run`.

Note: there is currently a bug which means that if the frontend is refreshed, the backend needs to be restarted. This is due to the channel type between the midi listening thread and the websocket publishing thread.

This might be fixed by transitioning to a one-to-many publisher.