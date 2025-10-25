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
