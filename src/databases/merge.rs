use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Merge {
    #[structopt(short = "m", long = "merge-with", value_name = "PATH")]
    merge_with: String,
    #[structopt(short = "o", long = "output", value_name = "PATH")]
    output_path: String,
    #[structopt(long = "overwrite-output")]
    overwrite_output: bool,
}
