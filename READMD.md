# Multi-threaded webserver

In this project I've implemented a multi-threaded webserver written in rust.

## What it does?
About to take in HTTP requests and parses them and is able to send back the correct html file response.

It utilises:
- a thread pool that allows it to handle multiple threads
- Workers
- Channels that pass threads from one thread to another

## How to run it

In order to run the webserver you can just run

```commandline
cargo run
```

This will build the crate and run the webserver.


## Quick start

Run the webserver using
```
cargo run
```

Go to: http://localhost:7878.

You should see the job being sent to one of the workers as a request comes in.

```
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
    Running `target/debug/webserver`
Worker 0 got a job; executing
```

Note: for a single request more than one job might come up this is because web browsers might be making more than one request e.g. they might make a request for the page but also for the favicon.ico icon for the browser tab.


If you go to http://localhost:7878/sleep, it will simulate a slow response from the server by causing the thread to sleep for 5 seconds.