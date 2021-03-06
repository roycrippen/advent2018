use itertools;
use itertools::Itertools;

fn main() {
    let xs: Vec<_> = include_str!("data.txt").lines().collect();

    println!("part a: {}", part_a(&xs));
    println!("part b: {}", part_b(&xs));
}

fn part_a(xs: &Vec<&str>) -> usize {
    let (twos, threes): (Vec<bool>, Vec<bool>) = xs.into_iter().map(occurences).unzip();
    let two = twos.iter().filter(|elt| **elt).count();
    let three = threes.iter().filter(|elt| **elt).count();
    two * three
}

fn part_b(xs: &Vec<&str>) -> String {
    for x in xs {
        for y in xs {
            match diff_by_one_char(&x, &y) {
                Some(v) => return v,
                None => continue,
            }
        }
    }
    "NO MATCH FOUND".to_string()
}

fn diff_by_one_char(s1: &str, s2: &str) -> Option<String> {
    if s1.len() != s2.len() {
        return None;
    }

    let res_str: String = s1
        .bytes()
        .into_iter()
        .zip(s2.bytes())
        .filter(|(e1, e2)| e1 == e2)
        .map(|(e1, _)| e1 as char)
        .collect();

    match s1.len() - res_str.len() {
        1 => Some(res_str),
        _ => None,
    }
}

fn occurences(s: &&str) -> (bool, bool) {
    let mut bs: Vec<_> = s.bytes().collect();
    bs.sort();
    let xs: Vec<usize> = bs
        .iter()
        .group_by(|elt| *elt)
        .into_iter()
        .map(|(_, group)| group.count())
        .filter(|count| *count == 2 || *count == 3)
        .collect();

    (xs.contains(&2), xs.contains(&3))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> Vec<&'static str> {
        vec![
            "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
        ]
    }

    #[test]
    fn test_occurances() {
        let xs: Vec<(bool, bool)> = get_test_data().iter().map(occurences).collect();
        let res = [
            (false, false),
            (true, true),
            (true, false),
            (false, true),
            (true, false),
            (true, false),
            (false, true),
        ];
        assert_eq!(xs, res);
    }

    #[test]
    fn test_part_a() {
        assert_eq!(part_a(&get_test_data()), 12);
    }

    #[test]
    fn test_part_b() {
        assert_eq!(part_b(&get_test_data()), "abcde".to_string());
    }
}
