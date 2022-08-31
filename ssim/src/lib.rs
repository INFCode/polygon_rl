use opencv::core::{Size, BORDER_DEFAULT};
use opencv::{imgproc::gaussian_blur, prelude::*};

// calculate the SSIM value comparing 2 grayscale image
fn ssim(image1: &[&[u8]], image2: &[&[u8]]) -> f32 {
    // 1. Initialize all required variables
    // The initial images
    let img1 = Mat::from_slice_2d(image1).unwrap();
    let img2 = Mat::from_slice_2d(image2).unwrap();

    // pixel-wise product of images (intermediate variables)
    let mut img11 = Mat::default();
    let mut img22 = Mat::default();
    let mut img12 = Mat::default();

    // The average of images (intermediate variables)
    let mut mu1 = Mat::default();
    let mut mu2 = Mat::default();

    // The products of averages
    let mut mu11 = Mat::default();
    let mut mu22 = Mat::default();
    let mut mu12 = Mat::default();

    // the squared variances of images
    let mut sigma11 = Mat::default();
    let mut sigma22 = Mat::default();

    // the covariance of images
    let mut sigma12 = Mat::default();

    // 2. Calculating all these variables
    // TODO
    gaussian_blur(&img1, &mut mu1, Size::new(11, 11), 1.5, 1.5, BORDER_DEFAULT).unwrap();
    gaussian_blur(&img2, &mut mu2, Size::new(11, 11), 1.5, 1.5, BORDER_DEFAULT).unwrap();
    // 3. The formula of SSIM
    // TODO
    0f32
}
