# Commit 1 Reflection Notes

In this commit, I learned how to:

- Set up a basic TCP server in Rust using `TcpListener`
- Handle incoming connections with the `handle_connection` function
- Parse HTTP requests using `BufReader`
- Collect and display HTTP request headers
- Debug errors in the build process including installing required build tools

The server currently listens on 127.0.0.1:7878 and outputs the request details to the console.

# Commit 2 Reflection Notes

In this commit, I enhanced the web server to return HTML content:

- Implemented the `handle_connection` function to properly process HTTP requests
- Used `BufReader` to efficiently read the request data from the TCP stream
- Collected HTTP headers from incoming requests (though not utilizing them yet)
- Created a proper HTTP response with status line ("HTTP/1.1 200 OK")
- Read HTML content from the file system using `fs::read_to_string`
- Added appropriate Content-Length header to the response
- Formatted a complete HTTP response with headers and body
- Successfully wrote the response back to the client using `stream.write_all`

The server now serves the contents of hello.html to any client that connects, regardless of the request path or method.

# Commit 3 Reflection Notes

In this commit, I implemented validation of incoming requests and selective responses:

- Added conditional logic to examine the HTTP request path
- Implemented path validation to check if the request is for "/hello"
- Created a branching response system using if-else statements
- Set up proper HTTP status codes (200 OK for valid paths, 404 NOT FOUND for invalid paths)
- Configured the server to serve different HTML files based on the request path
- Added a custom 404 page that displays when users request invalid routes
- Enhanced error handling when reading HTML files with unwrap_or_else
- Maintained the same response structure but with dynamic content based on request

This implementation creates a more realistic web server that can handle different routes and respond appropriately to both valid and invalid requests.

# Commit 4 Reflection Notes

In this commit, I simulated and observed the limitations of a single-threaded web server:

- Added code to artificially delay responses for the "/sleep" endpoint using `thread::sleep`
- The server waits for 10 seconds before responding to requests to "/sleep"
- Tested the server by opening two browser windows simultaneously
- Observed that when one browser requests "/sleep", the second browser's request to "/" is blocked
- Realized that a single-threaded server can only process one request at a time
- Identified a critical limitation: slow requests block all subsequent requests
- Understood why this behavior occurs: the server is using a single thread to handle all connections
- Recognized that this architecture would not scale in real-world applications with many users

This simulation demonstrates why multi-threading or asynchronous processing is essential for web servers that need to handle concurrent connections efficiently.

# Commit 5 Reflection Notes

In this commit, I implemented a multithreaded web server using a ThreadPool:

- Created a ThreadPool library to manage a pool of worker threads
- Used the `mpsc` (multi-producer, single-consumer) channel to communicate between threads
- Implemented a Job type as a boxed closure that can be executed by worker threads
- Used `Arc<Mutex<>>` to safely share the receiver between multiple threads
- Created worker threads that wait for jobs and execute them when received
- Modified the main server to use the ThreadPool for handling connections
- Implemented proper shutdown mechanism for the ThreadPool using the Drop trait
- Used message passing to communicate between the main thread and worker threads
- Verified that slow requests no longer block other requests from being processed
- Tested concurrent processing by accessing "/sleep" in one browser and "/" in another

This implementation demonstrates how concurrent programming can significantly improve the performance and responsiveness of a web server. The ThreadPool pattern allows the server to handle multiple connections simultaneously while limiting the total number of threads to a fixed size, preventing resource exhaustion.

# Commit Bonus Reflection Notes

In this bonus implementation, I improved the ThreadPool creation by implementing the Builder pattern:

- Created a `ThreadPoolBuilder` struct to replace direct instantiation via `ThreadPool::new()`
- Implemented a fluent interface with chainable methods (`size()`, `name()`)
- Added proper error handling with `Result<ThreadPool, String>` instead of panicking
- Made thread pool size a required parameter, enforced at compile time
- Added an optional parameter for naming the thread pool
- Provided better diagnostics with more specific error messages
- Added the ability to expand with more options in the future without breaking the API
- Enhanced the user experience by creating a more declarative configuration style
- Implemented a cleaner separation between configuration and object creation

Comparing the approaches:
1. **Constructor approach**: Simple but rigid - `let pool = ThreadPool::new(4);`
2. **Builder pattern**: Flexible and expressive - `let pool = ThreadPool::builder().size(4).name("WebServer").build()?;`

The builder pattern provides significant advantages in complex object creation scenarios by making the code more readable, maintainable, and adaptable to future requirements.