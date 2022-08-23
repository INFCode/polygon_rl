use super::polygon::{Polygon, ValidPolygon};
use crate::utils::Require;
use tiny_skia::{FillRule, Paint, Pixmap, Transform};

fn draw_polygon<const N: usize>(canvas: &mut Pixmap, polygon: &Polygon<N>)
where
    Require<{ N >= 3 }>: ValidPolygon,
{
    let path = polygon.boarder_path();
    if let Some(path) = path {
        let mut paint = Paint::default();
        let color = polygon.color();
        paint.set_color(color);
        canvas.fill_path(
            &path,
            &paint,
            FillRule::Winding,
            Transform::identity(),
            None,
        );
    }
}

#[cfg(test)]
mod test {
    use tiny_skia::Color;

    use super::*;

    #[test]
    fn draw_red_star() {
        let mut canvas = Pixmap::new(512, 512).unwrap();
        canvas.fill(Color::WHITE);

        // a red star centered at (256,256)
        let star_array: [f32; 14] = [
            // vertex 1
            134.2647659,
            216.4458247,
            // vertex 2
            377.7352341,
            216.4458247,
            // vertex 3
            180.7634877,
            359.5541753,
            // vertex 4
            256.0000000,
            128.0000000,
            // vertex 5
            331.2365123,
            359.5541753,
            // Color (rgba)
            1f32,
            0f32,
            0f32,
            0.75f32,
        ];

        let red_star = Polygon::<5>::from_array(&star_array);

        draw_polygon(&mut canvas, &red_star);

        canvas.save_png("./red_star.png").unwrap();
    }
}
