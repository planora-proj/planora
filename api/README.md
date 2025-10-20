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

## 🚀 Setup

#### 1. Environment Variables

The project uses `.env.local` files to manage environment variables for both the **Next.js frontend** and **Rust backend**.
Before running the application, copy the sample environment files:

```bash
    # From the project root
    cp .env.example .env.local
```

> ⚠️ Important:
Do not commit `.env.local` — it contains sensitive data.
Use `.env.example` to share safe configuration templates.

#### 2. Run locally

```bash
    cargo run
```
