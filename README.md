# Simple Rust HTTP Server

This repository contains a simple HTTP server written in Rust. The server uses a handmade thread pool to handle incoming TCP connections and serve HTTP responses.

## Features

- Multithreaded server using a thread pool
- Basic HTTP GET request handling
- Static file serving

## Project Structure

The project is divided into two main parts:

- `src/lib.rs`: Contains the implementation of the `ThreadPool` struct, which manages a pool of worker threads. Each worker thread waits for jobs (functions/closures) to be sent via a channel, and executes them when they arrive.
- `src/main.rs`: Contains the main server logic. It sets up a TCP listener, accepts incoming connections, and dispatches them to the thread pool for processing. It also includes a basic HTTP request handler.

## Purpose

This project was created for educational purposes. It serves as a practical example of a multithreaded HTTP server implemented in Rust. It is not intended for production use.

## Usage

To run the server, use the following command:

```bash
cargo run
```

The server will start listening on `127.0.0.1:7878`. You can test it by navigating to `http://127.0.0.1:7878` in your web browser, or using a tool like `curl`.

## Dependencies

- Rust programming language
- Cargo, Rust's package manager
