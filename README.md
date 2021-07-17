# warp-pprof

This is an example application to use [tikv/pprof-rs](https://github.com/tikv/pprof-rs) with [warp](https://github.com/seanmonstar/warp).

## Usage

Run webserver.

```sh
❯ cargo run
```

The server will start with 3030 port.

Fetch pprof.pb using curl.

```sh
❯ curl "http://localhost:3030/debug/pprof/profile?seconds=10" -o pprof.pb
```

This will take 10 seconds. During this period, send a dummy request runs a loop.

```sh
❯ curl "http://localhost:3030/dummy"
```

Then, visualize pprof.pb using `go tool pprof`.

```sh
go tool pprof -http=:8080 pprof.pb
```
