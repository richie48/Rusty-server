Http server in rust POC. This is a basic http server that listens on a port and handles incoming connections

## How to setup
Install rust from the official [website](https://www.rust-lang.org/tools/install) and add it to your path. 
From the root of the project run the below command to build the project
```bash
    cargo build
```

## How to run
Run `cargo run` in the project directory to start the server. You can then send requests to `http://localhost:4221` to see the server in action. For example, you can use `curl` to send a request:
```bash
    curl -v http://localhost:4221
    # Sent HTTP response: "HTTP/1.1 200 OK"

    curl -v http://localhost:4221/hello
    # Sent HTTP response: "HTTP/1.1 404 NOT FOUND"

    curl -v http://localhost:4221/echo/abc
    # Sent HTTP response: HTTP/1.1 200 OK
    # Content-Type: text/plain
    # Content-Length: 3

    # abc

    curl -v --header "User-Agent: agent/1.2" http://localhost:4221/user-agent
    # Sent HTTP response: HTTP/1.1 200 OK
    # Content-Type: text/plain
    # Content-Length: 9

    # agent/1.2
``` 
