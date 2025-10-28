# Planora Backend

The backend service for **Planora**, powering the web UI.  
Built with **Actix Web** for high-performance **HTTP** and **WebSocket** communication,  
and **PostgreSQL (SQLx)** for data persistence.

## 🧩 Overview

Planora Backend handles:
- REST APIs for the Planora web client
- WebSocket connections for real-time updates
- Authentication via JWT
- Metrics, traces, and logs through OpenTelemetry and Prometheus
- Database interactions using SQLx with PostgreSQL

## 🌐 Deployment

The API is deployed under [Planora API](https://api.planora.sbs/v1/health) and currently exposes the following public endpoint:

- `/v1/health` — returns a simple health check response to verify the API is running.

> Note: All other endpoints are protected or internal and not publicly exposed.

## ⚙️ Develoment Setup

#### 1. Prerequisites

Ensure you have the following installed:
- [Rust (latest stable)](https://rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/engine/install/)
- **SQLx CLI (for migrations)**

To install SQLx CLI:
```bash
    cargo install sqlx-cli --no-default-features --features native-tls,postgres
```

#### 2. Run PostgreSQL for Development

For local development, run a dedicated **Postgresql** container:
```bash
    docker run --name planora -d \
    -e POSTGRES_USER=planora \
    -e POSTGRES_PASSWORD=planora \
    -e POSTGRES_DB=planora \
    -p 5432:5432 \
    -v planora_pgdata:/var/lib/postgresql/data \
    postgres:latest
```
This creates a reusable local database instance named planora with persistent data stored in the `planora_pgdata` volume

#### 3. Configure Environment Variables

Copy the example environment configuration and update it as needed:

```bash
    # from the root
    cp .env.sample .env.local
```
Make sure your .env.local includes a valid database URL:
```bash
    PG_DATABASE_URL=postgres://planora:planora@localhost:5432/planora
```
> ⚠️ **Important**:
Do not commit `.env.local` — it contains sensitive data.
Use `.env.sample` to share safe configuration templates.

#### 4. Run Database Migrations

Once PostgreSQL is running and your .env.exa is configured, run the SQLx migrations:
```bash
    sqlx migrate run -D postgres://planora:planora@localhost:5432/planora
```
This applies all schema changes to your local development database.

#### 5. Start the Development Server

After the database is up and migrations are applied, start the backend:
```bash
    cargo run
```
> 💡 **Note:** Always make sure your PostgreSQL container is running before starting the backend.


## 🧠 Tips for Development

- If you change SQL schema, remember to run:
```bash
    sqlx migrate add <migration_name>
    sqlx migrate run
```

- If connection issues occur, ensure PostgreSQL is running and accessible at `localhost:5432`
