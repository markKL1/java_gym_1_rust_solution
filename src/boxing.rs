use crate::util::{Point, Minimum, CoordType};
use ::smallvec::SmallVec;
use ::std::fmt;

#[derive(Debug)]
struct BoundingBox {
    min_x: CoordType,
    max_x: CoordType,
    min_y: CoordType,
    max_y: CoordType,
    min_z: CoordType,
    max_z: CoordType,
}

#[derive(Debug, Clone)]
struct PointCubeXAssignment {
    point: Point,
    x_cube_nr: usize,
}

struct Cubes {
    rib_len: CoordType,
    x_cnt: usize,
    y_cnt: usize,
    z_cnt: usize,
    yz_cnt: usize,
    total_cnt: usize,
    data: Vec<Vec<PointCubeXAssignment>>
}

impl fmt::Debug for Cubes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        f.write_str("Cubes[")?;
        f.write_str(&format!("{}", self.x_cnt));
        f.write_str(" x ");
        f.write_str(&format!("{}", self.y_cnt));
        f.write_str(" x ");
        f.write_str(&format!("{}", self.z_cnt));
        f.write_str("]")
    }
}

impl BoundingBox {
    fn volume(&self) -> CoordType {
        (self.max_x - self.min_x) *
        (self.max_y - self.min_y) *
        (self.max_z - self.min_z)
    }

    fn calc_cubes(&self, rib_len: CoordType, point_cnt: usize) -> Cubes {
        let x_cnt = ((self.max_x - self.min_x) / rib_len).ceil() as usize;
        let y_cnt = ((self.max_y - self.min_y) / rib_len).ceil() as usize;
        let z_cnt = ((self.max_z - self.min_z) / rib_len).ceil() as usize;
        let yz_cnt = y_cnt * z_cnt;
        let total_cnt = x_cnt * yz_cnt;
        let yz_bin_expected_cnt = 2 * (1 + point_cnt / yz_cnt);
        //TODO @mark: TUNE the size of smallvec
        let data = vec![Vec::with_capacity(yz_bin_expected_cnt); yz_cnt];
        Cubes {
            rib_len,
            x_cnt,
            y_cnt,
            z_cnt,
            yz_cnt,
            total_cnt,
            data,
        }
    }

    fn get(&self) {

    }
}

fn find_extrema(points: &[Point]) -> BoundingBox {
    let mut bbox = BoundingBox {
        min_x : points[0].x,
        max_x : points[0].x,
        min_y : points[0].y,
        max_y : points[0].y,
        min_z : points[0].z,
        max_z : points[0].z,
    };
    for point in points {
        if point.x < bbox.min_x {
            bbox.min_x = point.x;
        }
        if point.x > bbox.max_x {
            bbox.max_x = point.x;
        }
        if point.y < bbox.min_y {
            bbox.min_y = point.y;
        }
        if point.y > bbox.max_y {
            bbox.max_y = point.y;
        }
        if point.z < bbox.min_z {
            bbox.min_z = point.z;
        }
        if point.z > bbox.max_z {
            bbox.max_z = point.z;
        }
    }
    bbox
}

// Minimum cube rib length to still find the nearest pair even if totally homogeneous
fn min_cube_size(bounding_box: &BoundingBox, point_cnt: usize) -> CoordType {
    (bounding_box.volume() / point_cnt as f64).cbrt()
}

#[allow(dead_code)]
pub fn boxing_ser(points: &mut [Point]) -> (Point, Point) {
    let bbox = find_extrema(points);
    let min_len = min_cube_size(&bbox, points.len());
    let box_size = min_len;
    let cubes = bbox.calc_cubes(box_size, points.len());
    println!("cubes: {:?}", cubes);



//    let mut minimum = Minimum::new(
//        points[0].clone(),
//        points[1].clone(),
//    );
//    for i in 0..points.len() {
//        let p1 = points[i];
//        for j in (i + 1)..points.len() {
//            let p2 = points[j];
//            if p1.dist2(&p2) < minimum.dist2 {
//                minimum = Minimum::new(
//                    p1.clone(),
//                    p2.clone(),
//                );
//            }
//        }
//    }
//    (minimum.point1, minimum.point2)
    panic!();
}
