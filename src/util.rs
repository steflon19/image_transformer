use anyhow::{Result, anyhow};
/// A utility function that takes in a comma separated string and returns a matrix/kernel
///
/// [`util::parse_csv`] takes a comma separated string `input_str` that satisfies `impl Into<String>` containing a matrix row by row
/// and returns a a result containing a `Vec<Vec<f32>>` containing the matrix.
pub fn parse_convolution_matrix(input_str: impl Into<String>) -> Result<Vec<Vec<f32>>> {
    // TODO: Should propagate parsing error
    let values: Vec<f32> = input_str
        .into()
        .split(',')
        .map(|s| s.parse::<f32>().expect("Matrix consists of valid numbers"))
        .collect();
    let size = (values.len() as f32).sqrt() as usize;

    match size * size == values.len() {
        true => Ok(values.chunks(size).map(|chunk| chunk.to_vec()).collect()),
        false => Err(anyhow!("Invalid matrix dimensions")),
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "-1,-1,-1,-1,8,-1,-1,-1,-1";
    const OUTPUT: [[f32; 3]; 3] = [[-1., -1., -1.], [-1., 8., -1.], [-1., -1., -1.]];

    #[test]
    fn test_parse_convolution_matrix() {
        let output = parse_convolution_matrix(INPUT);
        assert!(output.is_ok());
        let out_unwrapped = output.unwrap();

        let result = out_unwrapped
            .iter()
            .zip(OUTPUT.iter())
            .all(|(out_row, expect_row)| {
                out_row
                    .iter()
                    .zip(expect_row.iter())
                    .all(|(out, expect)| (out - expect).abs() < f32::EPSILON)
            });
        assert!(result)
    }

    #[test]
    fn test_parse_invalid_convolution_matrix() {
        let output = parse_convolution_matrix("-1,1,1");
        assert!(output.is_err());
    }

    // TODO: this should probably not actually panic
    #[test]
    #[should_panic]
    fn test_parse_panic() {
        let output_two = parse_convolution_matrix("a,b,c,d");
        assert!(output_two.is_err());
    }
}
