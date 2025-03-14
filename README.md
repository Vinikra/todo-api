# Todo API

A simple and efficient RESTful API for managing tasks, built with Rust. This project demonstrates core backend development skills, including CRUD operations, user authentication with JWT, and database integration using SQLite. It’s a great example of a beginner-friendly yet robust backend application suitable for a portfolio.

## Features
- User registration and login with secure password hashing (Argon2).
- JWT-based authentication for protected endpoints.
- Task management with CRUD operations (Create and Read implemented).
- SQLite database for persistent storage.
- Asynchronous programming with `tokio` and `axum`.

## Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (install via `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- A terminal or command-line interface.

## Setup Instructions
1. Clone the Repository:
   ```bash
   git clone https://github.com/<your-username>/todo-api.git
   cd todo-api
   ```
2. Build and Run:
   ```bash
   cargo run
   ```
   - The server will start on `http://localhost:3000`.
   - A SQLite database file (`tasks.db`) will be created automatically in the project directory.

## API Endpoints

### Public Endpoints

#### `POST /register` - Register a new user
**Request:**
```bash
curl -X POST http://localhost:3000/register \
-H "Content-Type: application/json" \
-d '{"username": "testuser", "password_hash": "password123"}'
```
**Response:**
```json
"User registered"
```
- **Status:** `200 OK` on success, `400 Bad Request` if username is taken.

#### `POST /login` - Authenticate a user and get a JWT token
**Request:**
```bash
curl -X POST http://localhost:3000/login \
-H "Content-Type: application/json" \
-d '{"username": "testuser", "password_hash": "password123"}'
```
**Response:**
```json
"eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..."
```
- **Status:** `200 OK` with token, `401 Unauthorized` if credentials are invalid.

### Protected Endpoints (Require JWT Token)

#### `POST /tasks` - Create a new task
**Request:**
```bash
curl -X POST http://localhost:3000/tasks \
-H "Content-Type: application/json" \
-H "Authorization: Bearer <token>" \
-d '{"title": "Estudar Rust", "status": "pending"}'
```
**Response:**
```json
{"id":1,"user_id":1,"title":"Estudar Rust","description":null,"status":"pending","due_date":null}
```
- **Status:** `200 OK` on success, `401 Unauthorized` if token is invalid.

#### `GET /tasks` - List all tasks for the authenticated user
**Request:**
```bash
curl -X GET http://localhost:3000/tasks \
-H "Authorization: Bearer <token>"
```
**Response:**
```json
[{"id":1,"user_id":1,"title":"Estudar Rust","description":null,"status":"pending","due_date":null}]
```
- **Status:** `200 OK` with task list, `401 Unauthorized` if token is invalid.

## Project Structure
```
todo-api/
├── Cargo.toml          # Dependencies and project metadata
├── src/
│   ├── main.rs         # Entry point, server setup
│   ├── models.rs       # Data models (Task, User, NewUser, NewTask)
│   ├── routes.rs       # API endpoints and handlers
│   ├── db.rs           # Database setup and queries
│   ├── auth.rs         # Authentication logic (JWT, password hashing)
│   └── middleware.rs   # Middleware for JWT authentication
└── tasks.db            # SQLite database (generated on first run)
```

## Technologies Used
- **Rust**: A systems programming language focused on safety and performance.
- **axum**: A modern, ergonomic web framework for building APIs.
- **sqlx**: An async SQL toolkit for Rust, used with SQLite for database operations.
- **jsonwebtoken**: Library for generating and validating JWT tokens.
- **argon2**: Secure password hashing algorithm.
- **tokio**: Asynchronous runtime for Rust.
- **serde**: Serialization/deserialization framework for JSON handling.

## Future Improvements
- Add endpoints for updating (`PUT /tasks/{id}`) and deleting (`DELETE /tasks/{id}`) tasks.
- Implement integration tests using Rust’s testing framework.
- Deploy the API to a cloud service (e.g., Render, Fly.io).
- Add input validation for task fields (e.g., max length for title).

## Contributing
Feel free to fork this repository, submit pull requests, or open issues with suggestions or bug reports!
