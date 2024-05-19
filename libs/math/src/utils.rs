use rand::{distributions::Standard, prelude::Distribution, random, Rng};
use strum_macros::EnumIter;

use crate::{Dot, UnitVec3, UnitVec3f, Vec3f};

#[derive(Copy, Clone, EnumIter, Debug)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Distribution<Axis> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Axis {
        match random::<u32>() % 3 {
            0 => Axis::X,
            1 => Axis::Y,
            2 => Axis::Z,
            _ => unreachable!(),
        }
    }
}

pub fn random_unit() -> UnitVec3f {
    loop {
        let rnd: Vec3f = random();
        if rnd.len() <= 1. {
            break rnd.to_unit();
        }
    }
}

pub fn random_on_hemisphere(normal: &UnitVec3f) -> UnitVec3f {
    let random = random_unit();
    if normal.dot(&random) >= 0. {
        random
    } else {
        -random
    }
}

pub fn random_in_unit_disk() -> Vec3f {
    loop {
        let rnd: Vec3f = random();
        if rnd.len_squared() <= 1. {
            break rnd;
        }
    }
}
