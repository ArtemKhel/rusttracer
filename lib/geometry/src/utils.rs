use crate::unit_vec::UnitVec;
use crate::vec::Vec3;
use crate::Dot;
use rand::Rng;

pub fn random_unit() -> UnitVec {
    let mut rng = rand::thread_rng();
    loop {
        let rnd: Vec3 = rng.gen();
        if rnd.len() <= 1. {
            break rnd.to_unit();
        }
    }
}
pub(crate) fn random_on_hemisphere(normal: &UnitVec) -> UnitVec {
    let random = random_unit();
    if normal.dot(random) >= 0. {
        random
    } else {
        -random
    }
}
