use super::data::Point;

pub fn calculate_euclidean_distance (point_1: &Point, point_2: &Point) -> f64 {
  let result = (point_1.x - point_2.x).powi(2) + (point_1.y - point_2.y).powi(2);
  result.sqrt()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn default_euclidean () {
    let point_1 = Point { x: 1.0, y: 1.0 };
    let point_2 = Point { x: 0.0, y: 1.0 };
    let distance = calculate_euclidean_distance(&point_1, &point_2);
    let expected = 1.0;
    assert_eq!(distance, expected);
  }
}