use itertools;
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let points: Vec<Point> = include_str!("data.txt")
        .lines()
        .map(|s| {
            let ys: Vec<&str> = s.split(", ").collect();
            Point::new(ys[0].parse().unwrap(), ys[1].parse().unwrap())
        })
        .collect();

    // let points: Vec<Point> = vec![
    //     Point::new(1, 1),
    //     Point::new(1, 6),
    //     Point::new(8, 3),
    //     Point::new(3, 4),
    //     Point::new(5, 5),
    //     Point::new(8, 9),
    // ];

    println!("part a: {}", part_a(&points)); // ?
    println!("part b: {}", part_b(&points)); // ?
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
}

fn build_grid(points: &Vec<Point>) -> HashMap<Point, usize> {
    let min_x = points.iter().min_by(|a, b| a.x.cmp(&b.x)).unwrap().x;
    let min_y = points.iter().min_by(|a, b| a.y.cmp(&b.y)).unwrap().y;
    let max_x = points.iter().max_by(|a, b| a.x.cmp(&b.x)).unwrap().x;
    let max_y = points.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y;

    // build hash map of (point, min distance) for all coordinates
    let mut grid: HashMap<Point, usize> = HashMap::new();
    for x in min_x..max_x + 1 {
        for y in min_y..max_y + 1 {
            let p = Point::new(x, y);
            let mut ds: Vec<(i32, usize)> = points
                .iter()
                .enumerate()
                .map(|(i, point)| (p.distance(point), i))
                .collect();
            ds.sort();
            if ds.get(0).unwrap().0 != ds.get(1).unwrap().0 {
                grid.insert(p, ds.get(0).unwrap().1);
            }
        }
    }

    // get list of infinite area ids
    let mut remove_ids: Vec<usize> = vec![];
    for (p, id) in grid.clone().iter() {
        if p.x == min_x || p.x == max_x || p.y == min_y || p.y == max_y {
            remove_ids.push(*id);
        }
    }
    remove_ids.sort();
    remove_ids.dedup();

    // remove the area ids from the hash map
    grid = grid
        .iter()
        .filter(|(_p, id)| !remove_ids.contains(id))
        .map(|(p, id)| (p.clone(), *id))
        .collect();

    grid
}

fn find_largest_area(grid: &HashMap<Point, usize>) -> usize {
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

fn part_b(points: &Vec<Point>) -> usize {
    points.len()
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
    fn test_part_a() {
        let points = get_test_points();
        assert_eq!(part_a(&points), 17);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(true, true);
    }
}
