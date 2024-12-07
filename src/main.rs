use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(short = 's', long, default_value = "img_sample")]
    img_sample_path: String,
    #[clap(short = 'i', long, default_value = "img_source")]
    img_source_path: String,
    #[clap(short = 'o', long, default_value = "img_result")]
    img_result_path: String,
    #[clap(short = 't', long, default_value_t = 10)]
    hamming_threshold: usize,
    #[clap(
        long,
        help = "If the --clean-flag is explicitly specified, incorrectly formatted images will be automatically deleted"
    )]
    clean_flag: bool,
}

fn main() {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    understood::run(
        &args.img_sample_path,
        &args.img_source_path,
        &args.img_result_path,
        args.hamming_threshold,
        args.clean_flag,
    );
}
