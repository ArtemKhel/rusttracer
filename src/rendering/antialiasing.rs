use itertools::Itertools;
use rand::random;

use crate::rendering::PixelCoord;

pub struct AntiAliasing {
    pub offsets: Vec<PixelCoord>,
}

impl Default for AntiAliasing {
    fn default() -> Self { AAType::None.into() }
}

impl From<AAType> for AntiAliasing {
    fn from(aa_type: AAType) -> Self {
        match aa_type {
            AAType::None => AntiAliasing {
                offsets: vec![[0., 0.]],
            },
            AAType::Random(n) => AntiAliasing {
                offsets: vec![[random::<f32>() - 0.5, random::<f32>() - 0.5]; n],
            },
            AAType::RegularGrid(n) => {
                assert!(n > 0);
                let offset = 1.0 / n as f32;
                let half = offset / 2.;
                AntiAliasing {
                    offsets: Vec::from_iter(
                        (0..n)
                            .cartesian_product(0..n)
                            .map(|(x, y)| [x as f32 * offset + half, y as f32 * offset + half]),
                    ),
                }
            }
        }
    }
}

pub enum AAType {
    None,
    Random(usize),
    RegularGrid(usize),
}
