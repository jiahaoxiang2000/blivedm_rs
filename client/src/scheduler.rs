// In Cargo.toml, ensure you have: client = { path = "../client" }
use models::BiliMessage;
use std::sync::Arc;

use crate::models;

/// Trait for event handlers (plugins) that process BiliMessage.
pub trait EventHandler: Send + Sync {
    fn handle(&self, msg: &BiliMessage);
}

/// Scheduling mode: Parallel or Sequential.
pub enum ScheduleMode {
    Parallel,
    Sequential,
}

/// Scheduler struct: manages event handlers and dispatches messages.
pub struct Scheduler {
    /// Each stage is a Vec of handlers to run in parallel; stages run sequentially.
    stages: Vec<Vec<Arc<dyn EventHandler>>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler { stages: Vec::new() }
    }

    /// Add a new stage (group of handlers to run in parallel)
    pub fn add_stage(&mut self, handlers: Vec<Arc<dyn EventHandler>>) {
        self.stages.push(handlers);
    }

    /// Add a single handler as a new sequential stage
    pub fn add_sequential_handler(&mut self, handler: Arc<dyn EventHandler>) {
        self.stages.push(vec![handler]);
    }

    /// Trigger all stages with the given BiliMessage.
    pub fn trigger(&self, msg: BiliMessage) {
        for stage in &self.stages {
            let mut handles = vec![];
            for handler in stage {
                let msg = msg.clone();
                let handler = Arc::clone(handler);
                handles.push(std::thread::spawn(move || {
                    handler.handle(&msg);
                }));
            }
            // Wait for all handlers in this stage to finish before next stage
            for handle in handles {
                let _ = handle.join();
            }
        }
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::models::BiliMessage;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::{mpsc, Arc, Mutex};

    struct AssertHandler {
        called: Arc<AtomicBool>,
        last_msg: Arc<Mutex<Option<BiliMessage>>>,
    }
    impl super::EventHandler for AssertHandler {
        fn handle(&self, msg: &BiliMessage) {
            self.called.store(true, Ordering::SeqCst);
            let mut lock = self.last_msg.lock().unwrap();
            *lock = Some(msg.clone());
        }
    }

    #[test]
    fn test_scheduler_with_mpsc_channel() {
        let (tx, rx) = mpsc::channel();
        let called = Arc::new(AtomicBool::new(false));
        let last_msg = Arc::new(Mutex::new(None));
        let handler = AssertHandler {
            called: Arc::clone(&called),
            last_msg: Arc::clone(&last_msg),
        };
        let mut scheduler = super::Scheduler::new();
        scheduler.add_sequential_handler(Arc::new(handler));

        // Send a test message
        let test_msg = BiliMessage::Danmu {
            user: "user1".to_string(),
            text: "hello".to_string(),
        };
        tx.send(test_msg.clone()).unwrap();

        // Simulate receiving and triggering
        if let Ok(msg) = rx.recv() {
            scheduler.trigger(msg);
        }

        // Assert handler was called and message matches
        assert!(called.load(Ordering::SeqCst), "Handler was not called");
        let lock = last_msg.lock().unwrap();
        assert!(lock.is_some(), "No message stored in handler");
        assert_eq!(lock.as_ref().unwrap(), &test_msg, "Message does not match");
    }

    #[test]
    fn test_scheduler_add_stage_and_sequential_handler() {
        use crate::models::BiliMessage;

        struct CounterHandler {
            counter: Arc<AtomicUsize>,
        }
        impl super::EventHandler for CounterHandler {
            fn handle(&self, _msg: &BiliMessage) {
                self.counter.fetch_add(1, Ordering::SeqCst);
            }
        }

        let counter1 = Arc::new(AtomicUsize::new(0));
        let counter2 = Arc::new(AtomicUsize::new(0));
        let counter3 = Arc::new(AtomicUsize::new(0));

        let handler1 = Arc::new(CounterHandler {
            counter: Arc::clone(&counter1),
        });
        let handler2 = Arc::new(CounterHandler {
            counter: Arc::clone(&counter2),
        });
        let handler3 = Arc::new(CounterHandler {
            counter: Arc::clone(&counter3),
        });

        let mut scheduler = super::Scheduler::new();
        // Add a parallel stage (handler1 and handler2)
        scheduler.add_stage(vec![handler1, handler2]);
        // Add a sequential stage (handler3)
        scheduler.add_sequential_handler(handler3);

        let test_msg = BiliMessage::Danmu {
            user: "user2".to_string(),
            text: "test".to_string(),
        };
        scheduler.trigger(test_msg);

        // Both handler1 and handler2 should be called once (parallel stage)
        assert_eq!(counter1.load(Ordering::SeqCst), 1, "Handler1 not called");
        assert_eq!(counter2.load(Ordering::SeqCst), 1, "Handler2 not called");
        // handler3 should be called once (sequential stage)
        assert_eq!(counter3.load(Ordering::SeqCst), 1, "Handler3 not called");
    }
}
