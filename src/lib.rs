//! An asynchronous runtime compatible with WebAssembly and non-WebAssembly targets.
//!
//! # Rationale
//!
//! When designing components and libraries that works on both WebAssembly targets backed by
//! JavaScript Runtime and non-WebAssembly targets with Native Runtimes. Developers usually face
//! challenges that requires applying multiple feature flags throughout their application:
//!
//! 1. Select I/O and timers that works with the target runtime.
//! 2. Native Runtimes usually require `Send` futures and WebAssembly types are usually `!Send`.
//!
//! # Implementation
//!
//! To alleviate these issues, Tokise implements a single-threaded runtime that executes `?Send`
//! (`Send` or `!Send`) futures.
//!
//! On platforms with multi-threading support, tokise spawns multiple independent runtimes
//! proportional to the CPU core number. When tasks are spawned with a runtime handle, it will
//! randomly select a worker thread from the internal pool. All tasks spawned with `spawn_local`
//! will run on the same thread as the thread the task was running. When the runtime runs in a
//! WebAssembly target, all tasks will be scheduled on the main thread.
//!
//! This runtime is designed in favour of IO-bounded workload with similar runtime cost.
//! When running I/O workloads, it would produce a slightly better performance as tasks are
//! never moved to another thread. However, If a worker thread is busy,
//! other threads will not be able to steal tasks scheduled on the busy thread.
//! When you have a CPU-bounded task where CPU time is significantly
//! more expensive, it should be spawned with a dedicated thread (or Web Worker) and communicates
//! with the application using channels.
//!
//! Tokise provides the following components:
//!
//! 1. A Task Scheduler that is capable of running non-Send tasks.
//! 2. A Timer that is compatible with the scheduler backend.
//! 3. Task Synchronisation Mechanisms.
//!
//! # Runtime Backend
//!
//! Tokise runtime is implemented with different runtimes depending on the target platform and can
//! use all features (timers / IO / task synchronisation) from the selected native runtime:
//!
//! - `wasm-bindgen-futures` (WebAssembly targets)
//! - `tokio` (non-WebAssembly targets)

#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_docs, missing_debug_implementations, elided_lifetimes_in_paths)]

pub mod fmt;
pub mod pinned;
pub mod time;

#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
#[path = "rt_wasm_bindgen/mod.rs"]
mod imp;
#[cfg(any(not(target_arch = "wasm32"), target_os = "wasi"))]
#[path = "rt_tokio/mod.rs"]
mod imp;

mod runtime;
pub use runtime::*;
