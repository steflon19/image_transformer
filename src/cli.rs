use std::path::PathBuf;
use structopt::StructOpt;

/// A wrapper struct to parse commmand line arguments
///
/// Using the `StructOpt` crate command line arguments are handled
/// with corresponding help and error messages.
/// [`CommandLineArgs::from_args()`] automatically parses the
/// arguments into the struct.
#[derive(Debug, StructOpt)]
#[structopt(name = "Image convolution program", about = "Applies a convolution matrix to an image")]
#[structopt(global_setting = structopt::clap::AppSettings::AllowNegativeNumbers)]
pub struct CliArguments {
    #[structopt(parse(from_os_str), default_value="./images/l.png")]
    pub input_file: PathBuf,

    #[structopt(short = "m", long = "matrix", help = "The convolution matrix as a comma-separated string, e.g. \"1,2,1,2,4,2,1,2,1\"", default_value="-1,-1,-1,-1,8,-1,-1,-1,-1")]
    pub matrix: String,

    #[structopt(parse(from_os_str), short = "o", long = "output", help = "The output destination, defaults to ./output/out with the same extension as the input file", default_value="./output/out")]
    pub output_file: PathBuf,
}