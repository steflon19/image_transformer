use anyhow::{Result, anyhow};
use structopt::StructOpt;
mod cli;
mod util;
mod img;

fn main() -> Result<()> {
    // Get args from cli
    let args = cli::CliArguments::from_args();
    
    // Parse the matrix string into a matrix of floats
    let matrix_str = args.matrix;
    let matrix = util::parse_convolution_matrix(&matrix_str)?;

    // Check input file path
    let image_path = match args.input_file.exists() {
        true => args.input_file,
        false => return Err(anyhow!("The provided file does not exist"))
    };
    let image_path_clone = image_path.clone();
    let extension = image_path_clone.extension().expect("Input file should have a valid extensions"); // not entirely true, .asdf also works but will break on opening the image

    let image = img::Image::new(image_path)?;
    // Apply the convolution matrix to the image
    let transformed_img = image.apply_convolution_matrix(&matrix);
    // Write the resulting image to file
    let mut out_file_name = args.output_file;
    // Set the same extension as the input file, image crate is smart enough to handle the extension correctly
    out_file_name.set_extension(extension);
    transformed_img.save(out_file_name).expect("Saving the file works");
    
    Ok(())
}