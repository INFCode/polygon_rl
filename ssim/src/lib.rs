/**
 * This file is an rust version of an opencv based SSIM impliment in C++:
 * https://github.com/jrmuizel/ssim/blob/master/SSIM.cpp
 */
use opencv::{core as cvcore, Error};
use opencv::{imgproc, prelude::*};

// calculate the SSIM value comparing 2 grayscale image
pub fn ssim(image1: &[&[u8]], image2: &[&[u8]]) -> Result<f32, Error> {
    // 1. Initialize all required variables
    // The initial images
    let img1 = Mat::from_slice_2d(image1).unwrap();
    let img2 = Mat::from_slice_2d(image2).unwrap();
    let size = img1.size().unwrap();

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

    // temporary variable for evaluating sigma
    let mut sigma_tmp = Mat::default();

    // the squared variances of images
    let mut sigma11 = Mat::default();
    let mut sigma22 = Mat::default();

    // the covariance of images
    let mut sigma12 = Mat::default();

    // 2. Calculating all variables above
    // TODO
    cvcore::pow(&img1, 2f64, &mut img11)?;
    cvcore::pow(&img2, 2f64, &mut img22)?;
    cvcore::mul_mat_mat(&img1, &img2)?.to_mat()?;

    imgproc::gaussian_blur(
        &img1,
        &mut mu1,
        cvcore::Size::new(11, 11),
        1.5,
        1.5,
        cvcore::BORDER_DEFAULT,
    )?;
    imgproc::gaussian_blur(
        &img2,
        &mut mu2,
        cvcore::Size::new(11, 11),
        1.5,
        1.5,
        cvcore::BORDER_DEFAULT,
    )?;

    cvcore::pow(&mu1, 2f64, &mut mu11)?;
    cvcore::pow(&mu2, 2f64, &mut mu22)?;
    cvcore::mul_mat_mat(&mu1, &mu2)?.to_mat()?;

    imgproc::gaussian_blur(
        &img11,
        &mut sigma_tmp,
        cvcore::Size::new(11, 11),
        1.5,
        1.5,
        cvcore::BORDER_DEFAULT,
    )?;
    cvcore::add_weighted(&sigma_tmp, 1f64, &mu11, -1f64, 0f64, &mut sigma11, -1)?;

    imgproc::gaussian_blur(
        &img22,
        &mut sigma_tmp,
        cvcore::Size::new(11, 11),
        1.5,
        1.5,
        cvcore::BORDER_DEFAULT,
    )?;
    cvcore::add_weighted(&sigma_tmp, 1f64, &mu22, -1f64, 0f64, &mut sigma22, -1)?;

    imgproc::gaussian_blur(
        &img12,
        &mut sigma_tmp,
        cvcore::Size::new(11, 11),
        1.5,
        1.5,
        cvcore::BORDER_DEFAULT,
    )?;
    cvcore::add_weighted(&sigma_tmp, 1f64, &mu12, -1f64, 0f64, &mut sigma12, -1)?;
    // 3. The formula of SSIM
    // TODO
    Ok(0f32)
}
