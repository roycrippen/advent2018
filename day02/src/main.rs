use itertools;
use itertools::Itertools;

fn main() {
    let xs: Vec<String> = include_str!("data.txt")
        .lines()
        .map(|elt| elt.to_string())
        .collect();

    println!("part a: {}", part_a(&xs));
    println!("part b: {}", part_b(&xs));
}

fn part_a(xs: &Vec<String>) -> usize {
    let (twos, threes): (Vec<bool>, Vec<bool>) = xs.iter().map(occurences).unzip();
    let two = twos.iter().filter(|elt| **elt).count();
    let three = threes.iter().filter(|elt| **elt).count();
    two * three
}

fn part_b(xs: &Vec<String>) -> String {
    for x in xs {
        for y in xs {
            match diff_by_one_char(&x, &y) {
                Some(v) => return v,
                None => continue,
            }
        }
    }
    "NO MATCH".to_string()
}

fn diff_by_one_char(s1: &String, s2: &String) -> Option<String> {
    if s1.len() != s2.len() {
        return None;
    }

    let cs1: Vec<char> = s1.chars().collect();
    let cs2: Vec<char> = s2.chars().collect();

    let res_str: String = cs1
        .iter()
        .zip(cs2.iter())
        .filter(|(e1, e2)| e1 == e2)
        .map(|(e1, _)| *e1)
        .collect();

    match s1.len() - res_str.len() {
        1 => Some(res_str),
        _ => None,
    }
}

fn occurences(s: &String) -> (bool, bool) {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();
    let xs: Vec<usize> = chars
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

    fn get_test_data() -> Vec<String> {
        vec![
            "abcdef".to_string(),
            "bababc".to_string(),
            "abbcde".to_string(),
            "abcccd".to_string(),
            "aabcdd".to_string(),
            "abcdee".to_string(),
            "ababab".to_string(),
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
