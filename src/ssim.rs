use opencv::quality::{QualitySSIM, QualitySSIMTraitConst};
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

    // temporary variables used when evaluating formula
    let mut temp1 = Mat::default();
    let mut temp2 = Mat::default();
    let mut temp3 = Mat::default();

    // Constants: k_1, k_2, C_1, C_2, L
    const L: u8 = u8::MAX; // we use u8 for pixels so L = 2^8 - 1
    const K1: f64 = 0.01;
    const K2: f64 = 0.03;
    // C_1 = (K_1 * L) ^ 2
    const C1: f64 = (K1 * L as f64) * (K1 * L as f64);
    // C_2 = (K_2 * L) ^ 2
    const C2: f64 = (K2 * L as f64) * (K2 * L as f64);

    // 2. Calculating all variables above
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
    // (2*mu1_mu2 + C1)
    cvcore::scale_add(&mu12, 2f64, &C1, &mut temp1)?;
    // (2*sigma12 + C2)
    cvcore::scale_add(&sigma12, 2f64, &C2, &mut temp2)?;
    // ((2*mu1_mu2 + C1).*(2*sigma12 + C2))
    // (mu1_sq + mu2_sq + C1)
    // (sigma1_sq + sigma2_sq + C2)
    // ((mu1_sq + mu2_sq + C1).*(sigma1_sq + sigma2_sq + C2))
    // ((2*mu1_mu2 + C1).*(2*sigma12 + C2))./((mu1_sq + mu2_sq + C1).*(sigma1_sq + sigma2_sq + C2))
    // TODO
    Ok(0f32)
}

pub fn cv_ssim(image1: &[&[u8]], image2: &[&[u8]]) -> Result<f32, Error> {
    let img1 = Mat::from_slice_2d(image1).unwrap();
    let img2 = Mat::from_slice_2d(image2).unwrap();
    let mut ssim = QualitySSIM::create(&img1)?;
    let result = QualitySSIMTrait::compute(&mut ssim, &img2)?;
    let mean = result.iter().sum::<f64>() as f32 / result.len() as f32;
    Ok(mean)
}

#[cfg(test)]
mod test {
    #[test]
    use tiny_skia::Pixmap;

    use super::cv_ssim;
    fn same_img() {
        let star = Pixmap::load_png("tests/img/red_star.png")?;
        let star = star.pixels();
        assert(cv_ssim(star.data(), star.data()), 1)
    }
    fn similar_img() {}
    fn different_img() {}
}
