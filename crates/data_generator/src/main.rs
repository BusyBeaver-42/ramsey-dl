use clap::Parser;
use data_generator::{Cli, get_run_fn_pointer};

fn main() {
    #[cfg(debug_assertions)]
    {
        use std::env;

        unsafe {
            // SAFETY: Only the main thread is running
            env::set_var("RUST_BACKTRACE", "1");
        }
    }

    let cli = Cli::parse();
    let run = get_run_fn_pointer(cli.problem, cli.colors);
    run(cli.output_file, cli.samples, cli.workers, cli.chunk_size);
}
