use std::time::Duration;

use pprof::protos::Message;
use serde::Deserialize;
use warp::{http::Response, Filter};

#[derive(Deserialize)]
struct DebugParams {
    seconds: Option<u64>,
    frequency: Option<i32>,
}

async fn dump_cpu_prof(seconds: u64, frequency: i32) -> pprof::Result<pprof::Report> {
    let guard = pprof::ProfilerGuard::new(frequency)?;
    async_std::task::sleep(Duration::from_secs(seconds)).await;
    guard.report().build()
}

async fn profile_cpu_handler(dp: DebugParams) -> Result<impl warp::Reply, warp::Rejection> {
    let seconds = match dp.seconds {
        Some(v) => v,
        None => 10,
    };

    let frequency = match dp.frequency {
        Some(v) => v,
        None => 99,
    };

    let report = match dump_cpu_prof(seconds, frequency).await {
        Ok(report) => report,
        Err(_) => return Err(warp::reject()),
    };

    let mut body: Vec<u8> = Vec::new();
    match report.pprof() {
        Ok(profile) => match profile.encode(&mut body) {
            Ok(()) => Ok(Response::builder()
                .header("Content-Type", "application/protobuf")
                .body(body)),
            Err(_) => Err(warp::reject()),
        },
        Err(_) => Err(warp::reject()),
    }
}

async fn dummy_handler() -> Result<impl warp::Reply, warp::Rejection> {
    let mut v = 0;
    for _ in 1..5000000 {
        v += 1;
    }

    Ok(format!("{} loop finished!", v))
}

#[tokio::main]
async fn main() {
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));
    let profile_cpu =
        warp::path!("debug" / "pprof" / "profile").and(warp::query().and_then(profile_cpu_handler));
    let dummy = warp::path!("dummy").and_then(dummy_handler);

    let routes = warp::get().and(hello.or(profile_cpu).or(dummy));

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
