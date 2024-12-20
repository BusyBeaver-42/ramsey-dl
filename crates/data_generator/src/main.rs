use clap::Parser;
use data_generator::{Cli, get_run_fn_pointer};

fn main() {
    let cli = Cli::parse();
    let run = get_run_fn_pointer(cli.problem, cli.colors);
    run(cli.output_file, cli.samples, cli.workers, cli.chunk_size);
}
