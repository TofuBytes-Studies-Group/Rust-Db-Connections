# Rust Database Integration – PostgreSQL, Redis, MongoDB

This repository demonstrates Rust integration with three different databases:

- **PostgreSQL**
- **Redis**
- **MongoDB**

It was developed as part of our exam project to explore various data storage strategies for different aspects of a Rust Bevy game.

## Setup

The original implementation is located in the main exam project repo:  
[SOF_EXAM_25 – DATABASE_EXAM](https://github.com/TofuBytes-Studies-Group/SOF_EXAM_25/tree/main/DATABASE_EXAM)

Inside that directory, you’ll find:

- A `docker-compose.yml` file to spin up all three databases easily.
- A setup guide for configuring **MongoDB sharding**, included in the `how_to_setup.md` file.

## Usage

Once the databases are up and running, the program runs a console-based menu that allows you to perform CRUD operations across all three databases.

Each database serves a different purpose, inspired by how they might be used in a game:

### PostgreSQL – Game Data
- Stores structured game-related data such as player names, stats (e.g., health), and other core gameplay elements.
- Best suited for relational data and long-term persistence.

### Redis – Scoreboard
- Implements a 24-hour rotating scoreboard.
- Displays the top 100 players.
- Automatically resets at midnight.
- Optimized for fast leaderboard operations.

### MongoDB – AI-Generated Lore
- Stores dynamically generated game world lore produced by AI.
- Suitable for storing flexible, unstructured content like text blobs or JSON documents.


