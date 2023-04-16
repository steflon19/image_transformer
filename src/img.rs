
use std::path::PathBuf;

use image::{DynamicImage, GenericImageView, Rgba, ImageBuffer};
use anyhow::Result;

/// Contains a [`image::DynamicImage`] that can be used to apply convolutions on
pub struct Image {
    img: DynamicImage
}

impl Image {
    /// Creates a new [`img::Image`] from a given `PathBuff`
    /// Returns an error if the file does not exist
    pub fn new(image_path: PathBuf) -> Result<Image> {
        // Use the image crate to read the image file
        let img = image::open(image_path)?;
        Ok(Image { img })
    }

    /// Applies a kernel to the [`img::Image`]
    /// 
    /// [`img::apply_convolution_matrix`] matrix slice as `&[Vec<f32>]`
    /// and returns an `ImageBuffer<Rgba<u8>, Vec<u8>>` containing the transformed image.
    pub fn apply_convolution_matrix(&self, matrix: &[Vec<f32>]) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let (width, height) = self.img.dimensions();
        let mut output = ImageBuffer::new(width, height);
        // Make sure not to overflow, skip the border of the image
        let padding = (matrix.len() / 2).try_into().expect("Matrix size fits into u32");

        // Iterate over each pixel, skipping the border
        for y in padding..(height - padding) {
            for x in padding..(width - padding) {
                let mut r = 0.0;
                let mut g = 0.0;
                let mut b = 0.0;
                let mut a = 0.0;

                // Calculate kernel factors and apply the kernel factor to each channel
                for j in 0..matrix.len() {
                    for i in 0..matrix.len() {
                        // Get pixels surrounding the current image pixel at x y
                        let pixel = self.img.get_pixel(x + (i as u32) - padding, y + (j as u32) - padding);
                        let factor = matrix[j as usize][i as usize];

                        r += factor * pixel[0] as f32;
                        g += factor * pixel[1] as f32;
                        b += factor * pixel[2] as f32;
                        a += factor * pixel[3] as f32;
                    }
                }

                // Apply the new color values to the current pixel, making sure not to overflow
                let r = r.clamp(0.0, 255.0) as u8;
                let g = g.clamp(0.0, 255.0) as u8;
                let b = b.clamp(0.0, 255.0) as u8;
                let a = a.clamp(0.0, 255.0) as u8;

                output.put_pixel(x, y, Rgba([r, g, b, a]));
            }
        }

        output
    }
}