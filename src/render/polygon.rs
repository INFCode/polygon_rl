use crate::utils::{Cond, Require};
use std::array;
use tiny_skia::{Color, Path, PathBuilder, Point};

#[derive(Debug, Clone, Copy)]
pub struct Polygon<const N: usize>
where
    Require<{ N >= 3 }>: ValidPolygon,
{
    vertices: [Point; N],
    color: Color,
}

pub trait ValidPolygon = Cond;

impl<const N: usize> Polygon<N>
where
    Require<{ N >= 3 }>: ValidPolygon,
{
    pub fn from_array(arr: &[f32; 2 * N + 4]) -> Self {
        let vertices = array::from_fn(|i| Point::from_xy(arr[2 * i], arr[2 * i + 1]));
        let color = Color::from_rgba(arr[2 * N], arr[2 * N + 1], arr[2 * N + 2], arr[2 * N + 3])
            .unwrap_or(Color::TRANSPARENT);

        Self { vertices, color }
    }

    pub fn boarder_path(self) -> Option<Path> {
        let mut builder = PathBuilder::with_capacity(N + 1, N);
        builder.move_to(self.vertices[0].x, self.vertices[0].y);
        for i in 1..N {
            builder.line_to(self.vertices[i].x, self.vertices[i].y)
        }
        // return an empty path if the path is not valid
        builder.finish()
    }

    pub fn color(self) -> Color {
        self.color
    }
}
