# Planora Playground

A lightweight playground inside the Planora project for testing new functionalities, experiments, and rapid prototyping.  
This service provides a simple **web frontend** served via **Express.js**.

---

## 🧩 Overview

- Serves a minimal frontend for experimenting with features before integrating into the main Planora web.
- Can be used for testing APIs, WebSocket connections, or UI components.
- Acts as a safe sandbox for trying out new ideas without affecting production or main development code.

---

## 🧱 Tech Stack

- **Frontend:** Basic HTML/JS served via Express  
- **Backend / Server:** Actix Web

---

## 🚀 Setup

### 1. Install dependencies

```bash
    cd playground
    pnpm install
```

### 2. Start the Development Server

```bash
    pnpm dev
```

## 🧠 Usage

- Open the playground in a browser and test APIs or frontend experiments.
- Connect it to the backend services (like Planora API) for testing endpoints or WebSocket functionality
- Use it as a sandbox — changes here do not affect production.

## Notes for Contributors

- This is a temporary/testing space, not intended for production use.
- Feel free to add experiments, small scripts, or UI mockups.
- Keep changes isolated to avoid breaking shared dependencies.
