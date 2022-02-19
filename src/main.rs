use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 's', long, default_value = "img_sample")]
    img_sample_path: String,

    #[clap(short = 'i', long, default_value = "img_source")]
    img_source_path: String,

    #[clap(short = 'o', long, default_value = "img_result")]
    img_result_path: String,

    #[clap(
        short,
        long,
        default_value_t = 0,
        help = "If n_workers is 0, the number of cpu cores is automatically used"
    )]
    n_workers: usize,

    #[clap(short = 't', long, default_value_t = 10)]
    hamming_threshold: usize,

    #[clap(
        long,
        help = "If --clean-flag is explicitly specified, images in the wrong format will be deleted automatically"
    )]
    clean_flag: bool,
}

fn main() {
    let args = Args::parse();
    understood::run(
        &args.img_sample_path,
        &args.img_source_path,
        &args.img_result_path,
        args.n_workers,
        args.hamming_threshold,
        args.clean_flag,
    );
}
