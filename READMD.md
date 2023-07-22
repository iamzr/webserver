# Multi-threaded webserver

In this project I've implemented a multi-threaded webserver written in rust.

## What it does?
This webserver is able to consume HTTP requests, parse them and is able to send back the correct html file response.

It utilises:
- a thread pool that allows it to handle multiple threads via workers
- Channels that pass threads from one thread to another

Since this a multi-threaded application it means that it is able to handle multiple requests concurrently in different threads.
In a single-threaded application, each request is processed sequentially.
The issue with this is that if a request takes a while to process, then the subsequent requests that come in cant be processed until the current request has been completed.
This would mean if could take a long time for a request to return if it was stuck behind some very long running requests.
In comparision, using multi-threading we can assign requests to be processed by different threads.
This means that if a thread takes a long time to process it's not an issue since if another request comes in it can be processed by another thread. Thus the slow-returning request aren't holding up subsequent requests.


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