# Connect 4 Agent

This project is a Connect 4 agent that uses a backend solver created in Rust and a frontend UI made with Vue.

## Features

- Backend Solver:
  - Implemented in Rust
  - Uses the Minimax algorithm with optimizations such as Alpha-Beta pruning, Iterative Deepening, Move Ordering, Transposition Table, and Opening Database
  - Provides efficient and intelligent move selection for the Connect 4 game

- Frontend UI:
  - Built with Vue.js
  - Provides a user-friendly interface for playing Connect 4 against the agent
  - Displays the game board, current player, and game status
  - Allows users to make moves and interact with the game

## Run with Docker (recommended)

To get started with the Connect 4 agent using Docker, follow these steps:

1. Clone the repository

2. Run docker compose:
    ```shell
    $ cd connect4
    $ docker-compose up
    ```

3. Open your web browser and navigate to http://localhost:8080 to access the Connect 4 game.

## Run Web App (without Docker)

To get started with the Connect 4 agent, follow these steps:

1. Clone the repository

2. Build and run the backend solver (in webserver mode):
    ```shell
    $ cd connect4/backend
    $ cargo run --release compiled_db.bin --webserver
    ```

3. In a new terminal, build and run the frontend UI:
    ```shell
    $ cd connect4/frontend
    $ npm install
    $ npm run dev
    ```

4. Open your web browser and navigate to http://localhost:8080 to access the Connect 4 game.

## Run just CLI Solver

1. Clone the repository

2. Build and run the backend solver:
    ```shell
    $ cd connect4/backend
    $ cargo run --release compiled_db.bin
    ```

## Acknowledgements
Heavily inspired by https://blog.gamesolver.org/
