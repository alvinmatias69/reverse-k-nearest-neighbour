mod lib;
use lib::data::Point;
use lib::rknn;
use std::time::SystemTime;

fn main() {
    let query_point: Point = Point { x: 5.0, y: 1.0 };
    let interest_points = vec![
        Point { x: 2.0, y: 1.0 },
        Point { x: 3.0, y: 1.0 },
        Point { x: 4.0, y: 1.0 },
        Point { x: 4.0, y: 1.0 },
        Point { x: 4.0, y: 1.0 },
        Point { x: 4.0, y: 1.0 },
        Point { x: 4.0, y: 1.0 },
        Point { x: 4.0, y: 1.0 },
    ];
    let object: Point = Point { x: 0.0, y: 1.0 };
    let k: usize = 6;

    let start = SystemTime::now();
    let found = rknn::compute(&query_point, &interest_points, &object, k);
    println!("{:?}", start.elapsed().unwrap());
}
