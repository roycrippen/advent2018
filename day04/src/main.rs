use regex::Regex;
#[macro_use]
extern crate lazy_static;
use itertools;
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let mut _xs: Vec<&str> = include_str!("data.txt").lines().collect();
    let trans_by_id = load_trans(&mut _xs);

    println!("part a: {}", part_a(&trans_by_id)); // 67558
    println!("part b: {}", part_b(&trans_by_id)); // 78990
}

#[derive(Debug, PartialEq, Clone)]
struct Tran {
    id: String,
    date: String,
    b_hour: u32,
    b_min: u32,
    f_min: u32,
    w_min: u32,
}

impl Tran {
    fn new(id: &str, date: &str, b_hour: u32, b_min: u32, f_min: u32, w_min: u32) -> Tran {
        Tran {
            id: id.to_string(),
            date: date.to_string(),
            b_hour: b_hour,
            b_min: b_min,
            f_min: f_min,
            w_min: w_min,
        }
    }

    fn sleep_mins(self) -> Mins {
        let mut mins: Mins = [0; 60];
        let s = self.f_min as usize;
        let e = self.w_min as usize;
        for i in s..e {
            mins[i] = 1;
        }
        mins
    }
}

type Mins = [usize; 60];

fn add_mins(a: &Mins, b: &Mins) -> Mins {
    let mut z: Mins = [0; 60];
    for i in 0..60 {
        z[i] = a[i] + b[i];
    }
    z
}

fn load_trans(xs: &mut Vec<&str>) -> HashMap<String, Vec<Tran>> {
    xs.sort();
    lazy_static! {
        static ref RE_BASE: Regex = Regex::new(r"[\[](.+) ([0-9]+):([0-9]+)[\]] (.*)").unwrap();
    }
    // hold trans data for 'begin' and 'falls' records to vec can be made in one pass
    let mut curr_id = "None";
    let mut curr_b_hour = 60;
    let mut curr_b_min = 60;
    let mut curr_f_min = 60;
    let mut trans: Vec<Tran> = Vec::new();
    let mut it = xs.iter();
    while let Some(x) = it.next() {
        match RE_BASE.captures(x) {
            Some(caps) => {
                let rest: Vec<&str> = caps.get(4).unwrap().as_str().split(" ").collect();
                let act_str = rest[0];
                let hour: u32 = caps.get(2).unwrap().as_str().parse().unwrap();
                let min: u32 = caps.get(3).unwrap().as_str().parse().unwrap();
                match act_str {
                    "Guard" => {
                        let id_pos = rest[1].find('#').unwrap() + 1;
                        curr_id = rest[1].get(id_pos..).unwrap();
                        curr_b_hour = hour;
                        curr_b_min = min;
                    }
                    "falls" => curr_f_min = min,
                    _ => {
                        let date: &str = caps.get(1).unwrap().as_str();
                        trans.push(Tran::new(
                            curr_id,
                            date,
                            curr_b_hour,
                            curr_b_min,
                            curr_f_min,
                            min,
                        ))
                    }
                };
            }
            None => panic!("error parsing data"),
        }
    }

    // order before creating hashmap by 'id'
    trans.sort_by(|a, b| {
        let compound_a = a.id.clone() + &a.date;
        let compound_b = b.id.clone() + &b.date;
        compound_a.cmp(&compound_b)
    });

    let trans_by_id: HashMap<String, Vec<Tran>> = trans
        .iter()
        .group_by(|elt| elt.id.clone())
        .into_iter()
        .map(|(id, group)| (id, group.cloned().collect()))
        .collect();

    trans_by_id
}

fn calc_sleep_mins(trans: &Vec<Tran>) -> u32 {
    trans.iter().map(|tran| tran.w_min - tran.f_min).sum()
}

fn find_max_sleep_minute(ts: &Vec<Tran>) -> (usize, usize) {
    let tot_mins = ts.iter().fold([0; 60], |acc, tran| {
        add_mins(&acc, &tran.clone().sleep_mins())
    });
    let (max_sleep_min, sum_max_mins) = tot_mins
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap();

    (max_sleep_min, *sum_max_mins)
}

fn part_a(trans_by_id: &HashMap<String, Vec<Tran>>) -> usize {
    // find guard with most sleep time
    let (id, _max_sleep) = trans_by_id
        .iter()
        .map(|(id, ls)| (id, calc_sleep_mins(&ls)))
        .max_by(|a, b| a.1.cmp(&b.1))
        .unwrap();

    let (max_sleep_min, _sum_max_mins) = find_max_sleep_minute(trans_by_id.get(id).unwrap());
    let id: usize = id.parse().unwrap();
    max_sleep_min * id
}

fn part_b(trans_by_id: &HashMap<String, Vec<Tran>>) -> usize {
    // find min where sleep sum is maximum for each guard
    let xs: Vec<_> = trans_by_id
        .iter()
        .map(|(id, ls)| (id, find_max_sleep_minute(&ls)))
        .map(|(id, (min, sum))| (id, min, sum))
        .collect();

    let (id, max_sleep_min, _sum) = xs.iter().max_by(|a, b| a.2.cmp(&b.2)).unwrap();

    let id: usize = id.parse().unwrap();
    max_sleep_min * id
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> Vec<&'static str> {
        vec![
            "[1518-11-03 00:05] Guard #10 begins shift",
            "[1518-11-03 00:24] falls asleep",
            "[1518-11-05 00:55] wakes up",
            "[1518-11-01 00:00] Guard #10 begins shift",
            "[1518-11-04 00:46] wakes up",
            "[1518-11-01 00:05] falls asleep",
            "[1518-11-01 00:55] wakes up",
            "[1518-11-01 00:25] wakes up",
            "[1518-11-01 23:58] Guard #99 begins shift",
            "[1518-11-05 00:03] Guard #99 begins shift",
            "[1518-11-02 00:40] falls asleep",
            "[1518-11-03 00:29] wakes up",
            "[1518-11-04 00:02] Guard #99 begins shift",
            "[1518-11-01 00:30] falls asleep",
            "[1518-11-04 00:36] falls asleep",
            "[1518-11-02 00:50] wakes up",
            "[1518-11-05 00:45] falls asleep",
        ]
    }

    #[test]
    fn test_parse_trans() {
        let mut xs = get_test_data();
        let trans_by_id = load_trans(&mut xs);
        assert_eq!(trans_by_id.len(), 2);
        assert_eq!(trans_by_id.get("10").unwrap().len(), 3);
        assert_eq!(trans_by_id.get("99").unwrap().len(), 3);
        let tran = Tran::new("99", "1518-11-02", 23, 58, 40, 50);
        assert_eq!(tran, trans_by_id.get("99").unwrap()[0]);
    }

    #[test]
    fn test_sleep_mins() {
        let mut xs = get_test_data();
        let trans_by_id = load_trans(&mut xs);
        assert_eq!(calc_sleep_mins(&trans_by_id.get("10").unwrap()), 50);
        assert_eq!(calc_sleep_mins(&trans_by_id.get("99").unwrap()), 30);
    }

    #[test]
    fn test_part_a() {
        let mut xs = get_test_data();
        let trans_by_id = load_trans(&mut xs);
        assert_eq!(part_a(&trans_by_id), 240);
    }

    #[test]
    fn test_part_b() {
        let mut xs = get_test_data();
        let trans_by_id = load_trans(&mut xs);
        assert_eq!(part_b(&trans_by_id), 4455);
    }
}
