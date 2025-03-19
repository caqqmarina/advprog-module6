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