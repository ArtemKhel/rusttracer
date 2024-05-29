use std::f32::consts::FRAC_1_PI;

use crate::{
    core::SurfaceInteraction,
    math::{
        utils::spherical_coordinates::{spherical_phi, spherical_theta},
        Normed, Transform, Transformable,
    },
    point2, Point2f, Point3f,
};

trait TextureMapping2D {
    fn map(&self, surf_int: SurfaceInteraction) -> TextureCoords2D;
}

trait TextureMapping3D {
    fn map(&self, surf_int: SurfaceInteraction) -> TextureCoords3D;
}

pub struct TextureCoords2D {
    st: Point2f,
    // ds_dx: f32,
    // ds_dy: f32,
    // dt_dx: f32,
    // dt_dy: f32,
}

pub struct TextureCoords3D {
    st: Point3f,
}

struct UVMapping {
    /// Scale u
    su: f32,
    /// Scale v
    sv: f32,
    /// Offset u
    du: f32,
    /// Offset v
    dv: f32,
}

impl TextureMapping2D for UVMapping {
    fn map(&self, surf_int: SurfaceInteraction) -> TextureCoords2D {
        let ds_dx = self.su * surf_int.du_dx;
        let ds_dy = self.su * surf_int.du_dy;
        let dt_dx = self.sv * surf_int.dv_dx;
        let dt_dy = self.sv * surf_int.dv_dy;

        let st = point2!(
            self.su * surf_int.interaction.uv.coords.x + self.du,
            self.sv * surf_int.interaction.uv.coords.y + self.dv
        );
        TextureCoords2D { st } //, ds_dx, ds_dy, dt_dx, dt_dy };
    }
}

pub struct SphericalMapping {
    render_to_texture: Transform<f32>,
}

impl TextureMapping2D for SphericalMapping {
    fn map(&self, surf_int: SurfaceInteraction) -> TextureCoords2D {
        let texture_point = surf_int.interaction.point.transform(&self.render_to_texture);
        let vec = texture_point.coords.to_unit();
        let st = point2!(spherical_theta(*vec) * FRAC_1_PI, spherical_phi(*vec) * FRAC_1_PI / 2.);
        TextureCoords2D { st }
    }
}
