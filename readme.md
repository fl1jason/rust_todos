# Simple TODO Application API

This is a simple TODO Application API built using Rust with Diesel ORM, coupled to a PostgreSQL database.

## API Endpoints

### Get All Todos

- **GET**: `http://localhost:8080/api/todos`

### Create a New Todo

- **POST**: `http://localhost:8080/api/todos`
- **Body**: `{"title": "Buy beer", "description": "A nice Camden Pale would be good!"}`

### Delete a Todo

- **DELETE**: `http://localhost:8080/api/todos/[id]`

### Update a Todo

- **PUT**: `http://localhost:8080/api/todos/[id]`

## Deployment

1. **Start the PostgreSQL Docker container**:

   ```bash
   docker-compose -f postgres.yaml up -d
   ```

2. **Run the migrations**:

   ```bash
   diesel migration run
   ```

3. **Run the application**:
   ```bash
   cargo run
   ```

## Notes

- Replace `[id]` with the actual ID of the Todo item in the endpoint URLs.
- Ensure Docker is installed and running on your machine to use the `docker-compose` command.
- Make sure you have Diesel CLI installed for running migrations.
