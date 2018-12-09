use regex::Regex;
#[macro_use]
extern crate lazy_static;
// use itertools;
// use itertools::Itertools;

#[derive(Debug, PartialEq, Clone)]
struct Rect {
    id: u32,
    x_top: u32,
    y_top: u32,
    x_bot: u32,
    y_bot: u32,
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
                let x_top: u32 = caps.get(2).map_or(0, |v| v.as_str().parse().unwrap());
                let y_top: u32 = caps.get(3).map_or(0, |v| v.as_str().parse().unwrap());
                let width: u32 = caps.get(4).map_or(0, |v| v.as_str().parse().unwrap());
                let height: u32 = caps.get(5).map_or(0, |v| v.as_str().parse().unwrap());
                Rect {
                    id: id,
                    x_top: x_top,
                    y_top: y_top,
                    x_bot: x_top + width,
                    y_bot: y_top + height,
                }
            }
            None => Rect {
                id: 0,
                x_top: 0,
                y_top: 0,
                x_bot: 0,
                y_bot: 0,
            },
        }
    }
}

fn main() {
    let rects: Vec<Rect> = include_str!("data.txt")
        .lines()
        .map(|s| Rect::new(s))
        .collect();

    println!("rects length: {}", rects.len());

    println!("part a: {}", part_a(&rects));
    println!("part b: {}", part_b(&rects));
}

fn part_a(_xs: &Vec<Rect>) -> usize {
    10
}

fn part_b(_xs: &Vec<Rect>) -> String {
    "NO MATCH FOUND".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> Vec<Rect> {
        vec![Rect::new("#123 @ 3,2: 5x4"), Rect::new("#123 @ 3,2: 5x4")]
    }

    #[test]
    fn test_new_rect_from_regex() {
        let rect = Rect::new("#123 @ 3,2: 5x4");
        let res = Rect {
            id: 123,
            x_top: 3,
            y_top: 2,
            x_bot: 8,
            y_bot: 6,
        };
        assert_eq!(rect, res);

        let rect = Rect::new("#123 @asasa 3,2: 5x4");
        let res = Rect {
            id: 0,
            x_top: 0,
            y_top: 0,
            x_bot: 0,
            y_bot: 0,
        };
        assert_eq!(rect, res)
    }

    #[test]
    fn test_part_a() {
        assert_eq!(part_a(&get_test_data()), 10);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(part_b(&get_test_data()), "NO MATCH FOUND".to_string());
    }
}
