/// A utility function that takes in a comma separated string and returns a matrix/kernel
/// 
/// [`util::parse_csv`] takes a comma separated string `input_str` that satisfies `impl Into<String>` containing a matrix row by row
/// and returns a `Vec<Vec<f32>>` containing the matrix.
pub fn parse_convolution_matrix(input_str: impl Into<String>) -> Option<Vec<Vec<f32>>> {
    let values: Vec<f32> = input_str
        .into()
        .split(',')
        .map(|s| s.parse::<f32>().unwrap())
        .collect();
    let size = (values.len() as f32).sqrt() as usize;


    match size * size == values.len() {
        true => Some(values.chunks(size).map(|chunk| chunk.to_vec()).collect()),
        false =>  None,
    }
}