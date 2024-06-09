mod cornell_box;
// mod cubes;
// mod spheres;
// mod teapot;

pub use cornell_box::cornell_box;

use crate::{math::Transform, point3, shapes::mesh::Triangle, Point3f};

// pub use cubes::cubes;
// pub use spheres::spheres;
// pub use teapot::teapot;
pub fn teapot_triangles(transform: Transform<f32>) -> Vec<Triangle> {
    let obj = obj::Obj::load("./data/teapot.obj").unwrap();
    let vertices: Vec<Point3f> = obj.data.position.iter().map(|x| point3!(x[0], x[1], x[2])).collect();
    let normals = obj.data.normal;
    let group = obj.data.objects.first().unwrap().groups.first().unwrap();

    let mut triangles: Vec<Triangle> = vec![];
    for x in group.polys.iter() {
        let a = x.0[0].0;
        let b = x.0[1].0;
        let c = x.0[2].0;

        triangles.push(Triangle::new(
            vertices[a],
            vertices[b] - vertices[a],
            vertices[c] - vertices[a],
            transform,
        ));
    }
    triangles
}
