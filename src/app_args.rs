use clap::Parser;
use num::complex::Complex64;

#[derive(Parser, Debug)]
pub struct AppArgs {
    /// image height, for example = 1920
    pub height: usize,

    /// image width, for example = 1080
    pub width: usize,

    /// Num threads to speed up computation. Default to std::thread::available_parallelism()
    #[arg(long)]
    pub num_threads: Option<usize>,

    /// Upper left complex point for computation
    /// # Example  
    /// --upper-left='-1.2+0.35i'
    #[clap(long, default_value = "-1.6+0.05i")]
    pub upper_left: Complex64,

    /// Lower right complex point for computation
    /// # Example
    /// --lower-right='-1+0.2i'
    #[clap(long, default_value = "-1.8-0.05i")]
    pub lower_right: Complex64,
}
