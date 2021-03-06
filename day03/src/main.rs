use regex::Regex;
#[macro_use]
extern crate lazy_static;
use std::collections::HashSet;

fn main() {
    let mut rects: Vec<Rect> = include_str!("data.txt")
        .lines()
        .map(|s| Rect::new(s))
        .collect();

    println!("part a: {}", part_a(&mut rects)); // 118223
    println!("part b: {}", part_b(&rects)); // 412
}

#[derive(Debug, PartialEq, Clone)]
struct Rect {
    id: u32,
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
    overlap: bool,
}

impl Rect {
    fn new(claim: &str) -> Rect {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"#([0-9]+) @ ([0-9]+),([0-9]+): ([0-9]+)x([0-9]+)").unwrap();
        }

        match RE.captures(claim) {
            Some(caps) => {
                let id: u32 = caps.get(1).map_or(0, |v| v.as_str().parse().unwrap());
                let x1: u32 = caps.get(2).map_or(0, |v| v.as_str().parse().unwrap());
                let y1: u32 = caps.get(3).map_or(0, |v| v.as_str().parse().unwrap());
                let width: u32 = caps.get(4).map_or(0, |v| v.as_str().parse().unwrap());
                let height: u32 = caps.get(5).map_or(0, |v| v.as_str().parse().unwrap());
                Rect {
                    id: id,
                    x1: x1,
                    y1: y1,
                    x2: x1 + width - 1,
                    y2: y1 + height - 1,
                    overlap: false,
                }
            }
            None => Rect {
                id: 0,
                x1: 0,
                y1: 0,
                x2: 0,
                y2: 0,
                overlap: false,
            },
        }
    }

    fn overlaps(&self, other: &Rect) -> bool {
        let no_overlap =
            self.x1 > other.x2 || other.x1 > self.x2 || self.y1 > other.y2 || other.y1 > self.y2;
        !no_overlap
    }

    fn make_set(&mut self) -> HashSet<(u32, u32)> {
        // if we are here then the claim overlaps some other
        self.overlap = true;
        let mut set = HashSet::new();
        for i in self.x1..(self.x2 + 1) {
            for j in self.y1..(self.y2 + 1) {
                set.insert((i, j));
            }
        }
        set
    }
}

fn part_a(recs: &mut Vec<Rect>) -> usize {
    let mut xs = Vec::new();
    for i in 0..recs.len() {
        for j in (i + 1)..recs.len() {
            if recs[i].overlaps(&recs[j]) {
                let s1 = recs[i].make_set();
                let s2 = recs[j].make_set();
                let mut intersection: Vec<(u32, u32)> = s1.intersection(&s2).cloned().collect();
                xs.append(&mut intersection);
            }
        }
    }
    xs.sort();
    xs.dedup();
    xs.len()
}

fn part_b(xs: &Vec<Rect>) -> u32 {
    match xs.iter().find(|rect| !rect.overlap) {
        Some(rect) => rect.id,
        None => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> Vec<Rect> {
        vec![
            Rect::new("#1 @ 1,3: 4x4"),
            Rect::new("#2 @ 3,1: 4x4"),
            Rect::new("#3 @ 5,5: 2x2"),
        ]
    }

    #[test]
    fn test_new_rect_from_regex() {
        let rect = Rect::new("#123 @ 3,2: 5x4");
        let res = Rect {
            id: 123,
            x1: 3,
            y1: 2,
            x2: 7,
            y2: 5,
            overlap: false,
        };
        assert_eq!(rect, res);

        let rect = Rect::new("#123 @asasa 3,2: 5x4");
        let res = Rect {
            id: 0,
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
            overlap: false,
        };
        assert_eq!(rect, res)
    }

    #[test]
    fn test_part_a() {
        assert_eq!(part_a(&mut get_test_data()), 4);
    }

    #[test]
    fn test_part_b() {
        let mut xs = get_test_data();
        part_a(&mut xs);
        assert_eq!(part_b(&xs), 3);
    }
}
