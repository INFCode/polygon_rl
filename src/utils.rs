use ndarray::{Array, Ix, ShapeError};
use tiny_skia::Pixmap;
// Helper type & trait around to provide a clearer compile error message
// and as well a clearer way to express extra bounds for const generics
pub struct Require<const C: bool>;
pub trait Cond {}
impl Cond for Require<true> {}

fn Pixmap_to_slice(pixmap: &Pixmap) -> Result<&[&[u8]], ShapeError> {
    let shape = (pixmap.width() as Ix, pixmap.height() as Ix);
    let pixmap = pixmap.pixels();
    let pix_array = Array::from_shape_vec(shape, pixmap.to_vec())?;
    let red = pix_array.map(|pix| pix.red());
    let green = pix_array.map(|pix| pix.green());
    let blue = pix_array.map(|pix| pix.blue());
}
