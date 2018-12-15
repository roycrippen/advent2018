use itertools;
use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let points: Vec<Point> = include_str!("data.txt")
        .lines()
        .map(|s| {
            let ys: Vec<i32> = s.split(", ").map(|s| s.parse().unwrap()).collect();
            Point::new(ys[0], ys[1])
        })
        .collect();

    println!("part a: {}", part_a(&points)); // 3293
    println!("part b: {}", part_b(&points, 10000)); // 45176
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }

    fn distance(&self, other: &Point) -> i32 {
        i32::abs(self.x - other.x) + i32::abs(self.y - other.y)
    }

    fn total_distance(&self, others: &Vec<Point>) -> i32 {
        others
            .iter()
            .fold(0, |acc, other| acc + self.distance(other))
    }
}

fn get_bounds(points: &Vec<Point>) -> (i32, i32, i32, i32) {
    let min_x = points.iter().min_by(|a, b| a.x.cmp(&b.x)).unwrap().x;
    let min_y = points.iter().min_by(|a, b| a.y.cmp(&b.y)).unwrap().y;
    let max_x = points.iter().max_by(|a, b| a.x.cmp(&b.x)).unwrap().x;
    let max_y = points.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y;

    (min_x, min_y, max_x, max_y)
}

fn build_grid(points: &Vec<Point>) -> Vec<(Point, usize)> {
    // build hash map of (point, min distance) for all coordinates
    let (min_x, min_y, max_x, max_y) = get_bounds(&points);
    let mut grid: Vec<(Point, usize)> = vec![];
    for x in min_x..max_x + 1 {
        for y in min_y..max_y + 1 {
            let p = Point::new(x, y);
            let mut ds: Vec<(i32, usize)> = points
                .iter()
                .enumerate()
                .map(|(i, point)| (p.distance(point), i))
                .collect();
            ds.sort();
            let (first_distance, first_id) = ds.get(0).unwrap();
            let (second_distance, _second_id) = ds.get(1).unwrap();
            if first_distance != second_distance {
                grid.push((p, *first_id));
            }
        }
    }

    // get list of infinite area ids
    let x_bounds = [min_x, max_x];
    let y_bounds = [min_y, max_y];
    let remove_ids: HashSet<usize> = grid
        .iter()
        .filter(|(p, _id)| x_bounds.contains(&p.x) || y_bounds.contains(&p.y))
        .map(|(_p, id)| *id)
        .collect();

    // remove the area ids from the hash map
    grid = grid
        .iter()
        .filter(|(_p, id)| !remove_ids.contains(id))
        .map(|(p, id)| (p.clone(), *id))
        .collect();

    grid
}

fn find_largest_area(grid: &Vec<(Point, usize)>) -> usize {
    let mut xs: Vec<usize> = grid.iter().map(|(_point, id)| *id).collect();
    xs.sort();
    let xs: Vec<(usize, usize)> = xs
        .iter()
        .group_by(|id| *id)
        .into_iter()
        .map(|(id, group)| (*id, group.count()))
        .collect();

    let (_id, count) = xs.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();

    *count
}

fn part_a(points: &Vec<Point>) -> usize {
    let grid = build_grid(&points);
    let res = find_largest_area(&grid);

    res
}

fn part_b(points: &Vec<Point>, dist: i32) -> usize {
    let (min_x, min_y, max_x, max_y) = get_bounds(&points);
    let res = (min_x..max_x)
        .flat_map(|x| {
            (min_y..max_y).filter(move |y| Point::new(x, *y).total_distance(&points) < dist)
        })
        .count();

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_points() -> Vec<Point> {
        let points: Vec<Point> = vec![
            Point::new(1, 1),
            Point::new(1, 6),
            Point::new(8, 3),
            Point::new(3, 4),
            Point::new(5, 5),
            Point::new(8, 9),
        ];
        points
    }

    #[test]
    fn test_total_distance() {
        let points = get_test_points();
        let test_point = Point::new(4, 3);
        assert_eq!(test_point.total_distance(&points), 30);
    }

    #[test]
    fn test_part_a() {
        let points = get_test_points();
        assert_eq!(part_a(&points), 17);
    }

    #[test]
    fn test_part_b() {
        let points = get_test_points();
        assert_eq!(part_b(&points, 32), 16);
    }
}
