use super::data::Point;
use super::common;

pub fn compute(
  query_point: &Point,
  interest_points: &Vec<Point>,
  object: &Point,
  k: usize,
) -> bool {
  let query_distance: f64 = common::calculate_euclidean_distance(&query_point, &object);
  let mut distances = vec![query_distance];
  for point in interest_points.into_iter() {
    distances.push(common::calculate_euclidean_distance(&point, &object));
  }
  distances.sort_by(|a, b| a.partial_cmp(b).unwrap());

  let mut found = false;
  let mut index = 0;

  while !found && index < k {
    if distances[index] == query_distance {
      found = true;
    } else {
      index = index + 1;
    }
  }

  found
}
