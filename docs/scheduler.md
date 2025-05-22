# Scheduler/Event Trigger System

The scheduler module provides a flexible event trigger system for handling Bilibili live messages (`BiliMessage`). It supports both parallel and sequential event handler execution using a staged architecture.

## Key Concepts

- **EventHandler Trait**: Implement this trait to define custom handlers for `BiliMessage` events.
- **Scheduler**: Manages event handler registration and message dispatch. Handlers are organized into stages:
    - Each stage is a group of handlers run in parallel.
    - Stages themselves are executed sequentially.
- **API**:
    - `add_stage(Vec<Arc<dyn EventHandler>>)` — Add a parallel stage.
    - `add_sequential_handler(Arc<dyn EventHandler>)` — Add a single handler as a sequential stage.
    - `trigger(BiliMessage)` — Dispatch a message through all stages.

## Example Usage

```rust
let mut scheduler = Scheduler::new();
// Add two handlers to run in parallel
scheduler.add_stage(vec![Arc::new(MyHandler1), Arc::new(MyHandler2)]);
// Add a sequential handler
scheduler.add_sequential_handler(Arc::new(MyHandler3));

// When a BiliMessage is received:
scheduler.trigger(msg);
```

## Test Coverage
- The scheduler is tested with both parallel and sequential handler registration.
- Integration with `mpsc::channel` is verified for real-world message passing scenarios.

This design ensures extensibility and robust event-driven processing for Bilibili live room applications.
