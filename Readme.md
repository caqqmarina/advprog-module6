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