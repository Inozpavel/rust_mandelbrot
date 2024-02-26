use std::str::FromStr;

use anyhow::Context;
use clap::Parser;
use mimalloc::MiMalloc;
use tracing::debug;
use tracing_subscriber::EnvFilter;

use mandelbrot::{par_render, write_image};
use mandelbrot::app_args::AppAgrs;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_str(&std::env::var("RUST_LOG").unwrap_or("info".to_string()))?).init();
    let args = AppAgrs::parse();
    let parallelism = std::thread::available_parallelism().context("Get available parallelism")?;
    let mut parallelism = parallelism.get();
    
    debug!("App args: {args:?}");
    debug!("Available parallelism: {}", parallelism);

    let mut pixels = vec![0; args.height * args.width];
    if let Some(threads_count) = args.num_threads {
        rayon::ThreadPoolBuilder::new().num_threads(threads_count).build_global()?;
        parallelism = threads_count;
    }

    let bounds = (args.height, args.width);

    par_render(
        parallelism,
        255,
        &mut pixels,
        bounds,
        args.upper_left,
        args.lower_right,
    );

    write_image(&format!("./result-{}x{}.png", args.height, args.width), &pixels, bounds)?;

    Ok(())
}
