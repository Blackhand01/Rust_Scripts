# Rust Scripts

Welcome to my Rust Scripts repository! This is a collection of exercises I've completed to practice and deepen my understanding of Rust. Each exercise is contained within its own folder, with a brief description provided below.

## Battleship

In the `battleship` directory, I implemented a program that simulates the creation and management of a 20x20 Battleship game board. The board is stored in a file, and the program allows for the safe addition of ships, ensuring they do not overlap or touch each other. I focused on handling file manipulation and command-line interface parsing using the `clap` crate.

## DNA Sequence Matcher

The `DNA_sequence` folder contains my exploration of searching for specific DNA subsequences within larger DNA strings. I worked on implementing various search strategies, including the use of lambda functions, simple iterators, and Rust-compliant iterators. This exercise helped me gain a deeper understanding of Rust's lifetime annotations and iterator patterns.

## File System Simulator

In the `file_system` directory, I developed an in-memory file system that supports operations like creating, removing, and updating files and directories. The focus was on implementing a tree structure to manage the file system's hierarchy and ensuring that all operations adhered to Rust's strict ownership and borrowing rules. This exercise was particularly valuable for understanding how to work with mutable references and lifetime annotations in complex data structures.

## Circular Buffer

The `circular_buffer` folder contains an implementation of a circular (ring) buffer, a fixed-size data structure that operates in a FIFO (First In, First Out) manner. This exercise helped me explore Rust's handling of collections, particularly with regards to borrowing, ownership, and mutability. The implementation ensures that the buffer is thread-safe, meaning that the `read` and `write` operations can be safely called from different threads. This exercise deepened my understanding of concurrency in Rust, specifically around managing access to shared resources between threads.

## Cyclic Barrier Mutex

In the `cyclic_barrier_mutex` directory, I explored the concept of a cyclic barrier, a synchronization primitive that allows multiple threads to wait for each other to reach a common point before proceeding. The exercise involved implementing a cyclic barrier that could be reused multiple times within a loop, with careful attention to ensuring that fast threads do not re-enter the barrier while slower threads are still exiting. I used condition variables to manage the barrier's state transitions and ensure proper synchronization between threads. This exercise provided valuable insights into more complex concurrency patterns in Rust and the importance of condition variables in ensuring safe and predictable thread synchronization.

## Cyclic Barrier Channel

In the `cyclic_barrier_channel` directory, I implemented a cyclic barrier using Rust's channels for synchronization instead of shared state protected by a mutex. Each thread receives a `Waiter` object, which contains multiple sender channels and a single receiver. The `Waiter` object’s `wait()` method sends a message on each sender and then waits to receive messages from the receiver. This approach allowed me to explore channel-based synchronization, which avoids some of the pitfalls of shared state, such as potential deadlocks. The exercise also provided a deeper understanding of how Rust’s type system and ownership model can help manage complex synchronization patterns without requiring explicit locking mechanisms.

## Threadpool

The `threadpool` directory contains an implementation of a thread pool, a structure that maintains a pool of worker threads ready to execute incoming jobs. The `ThreadPool` struct has an `execute` method, which accepts a job (a closure encapsulated in a `Box<dyn FnOnce() + Send>`) and schedules it for execution by one of the worker threads. If all workers are busy, the job is queued until a worker becomes available. This exercise helped me understand the internal workings of thread pools, particularly around job scheduling, managing worker threads, and ensuring that the system remains responsive even under heavy load. As a bonus, I implemented a `stop()` method to gracefully shut down the pool by waiting for all workers to finish their current tasks.

## Downloader

In the `downloader` directory, I created a `Downloader` object designed to handle network downloads with a timeout. The `Downloader` spawns a child process using `curl` to fetch the content from a given URL. If the download exceeds the specified timeout, the process is terminated. This exercise was particularly useful for understanding how to manage long-running operations in a child process and ensure they don’t hang indefinitely. Additionally, I explored the use of Rust’s `thread::sleep` and process management features to handle timeouts and resource cleanup. The implementation can also be extended by integrating with the thread pool from the previous exercise, allowing multiple downloads to be managed concurrently without blocking the main application thread.

---

Each of these exercises has been instrumental in deepening my understanding of Rust, particularly in areas such as memory safety, concurrency, and systems programming. I hope you find these examples useful and informative as you explore the powerful features of the Rust programming language.
