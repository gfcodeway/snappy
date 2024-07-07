# Snappy Minimal Power Server

A lightweight and efficient web server built with Actix Web for serving static files with intelligent routing and caching.

## Features

- Serves static files from the `./dist` directory
- Intelligent routing: Falls back to `index.html` for non-existent files (useful for single-page applications)
- Caching strategy for JavaScript and CSS files
- Logging of server activities

## Prerequisites

- Rust programming language
- Cargo package manager

## Dependencies

- actix-web
- actix-files
- env_logger
- log

## Installation

1. Clone the repository:
2. Build the project:

## Usage

1. Place your static files in the `./dist` directory.

2. Run the server:
3. The server will start and listen on `http://127.0.0.1:8080`.

## Configuration

- The server listens on `127.0.0.1:8080` by default. Modify the `bind` address in the `main` function to change this.
- Static files are served from the `./dist` directory. Update the `file_path` in the `serve_file` function to change this.
- Cache-Control headers are set to `public, max-age=3600` for JavaScript and CSS files. Adjust this in the `serve_file` function as needed.

## Logging

- The server uses `env_logger` for logging.
- Set the `RUST_LOG` environment variable to control log levels (e.g., `RUST_LOG=debug`).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is open-source and available under the [MIT License](LICENSE).