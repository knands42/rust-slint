# Sample Project

A local-first desktop app built in **Rust + Slint**.

## What it does

#TODO

## Architecture

```
┌──────────────────────────────┐
│  Slint UI (Rust frontend)    │
│  main · login · admin_page   │
└──────────────┬───────────────┘
               │ messages/events (async channels)
┌──────────────┴───────────────┐
│  appliacation manager        │
└──────┬───────┬───────┬───────┘
       │       │       │
┌──────┴───────┴───────┴────────┐
│ domain layer                  │
│ persistence layer             │
└───────────────────────────────┘
```

## Getting started

### Prerequisites

Install pkg-config:

```shell
sudo pacman -S pkgconf fontconfig libxkbcommon
```

### Execute the application
```bash
# prerequisites: Rust (stable), a local LLM runtime (Ollama or llama.cpp server)
cargo run
```

## AI usage in this project

No vibecoding, any bug this project has, it's entirely my fault. AI is used deliberately for:

- Getting unstuck on specific implementations (still learning Rust)
- Troubleshooting (tons of UI wiring and state management)
- Helping to write tests
- Complex layouts (I not good with UI development)
- Initial UI mockups — a starting point

## Status

Early stage. Rust + Slint, design system and view specs live in [`docs/`](docs/).
