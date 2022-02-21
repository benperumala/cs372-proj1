# README

This project uses the rust Crate [Actix Web](https://actix.rs/). Crates are public libraries that can be installed through the internet like with Python's `pip` or NodeJS' `npm`. With Actix Web, it is possible to create an asynchronous HTTP server which is able to efficiently process requests while keeping system resources down to a minimum.

This project is split into [main.rs](src/main.rs) and [db.r](src/db.rs). main.rs is the primary web server while db.rs acts as a simple JSON file-backed database.

When you run the project via `cargo run`, a new HTTP server will be started on <http://127.0.0.1:8080> with a few routes.

All the routes below are relative paths. To get the absolute path, prepend the HTTP link above:

- `/` - Homepage. Rust will grab a static HTML file from the filesystem and send that to the user
- `/songs` - Get all songs in the database
- `/songs/<id>` - Get a single song from the master list where `<id>` is the index
- `/echo` - A POST only route that will echo back a message that is sent to it

Going to <http://127.0.0.1/> will open up the homepage which will:

- Allow you to test the `/echo` route
- Populate a table with all known songs from the database (via javascript request)