use itertools;
use itertools::Itertools;

fn main() {
    let xs: Vec<String> = include_str!("data.txt")
        .lines()
        .map(|elt| elt.to_string())
        .collect();

    let test_data = get_test_data();
    println!("part a: {}", part_a(&xs));
    println!("part b: {}", part_b(&test_data));
}

fn part_a(xs: &Vec<String>) -> usize {
    let (twos, threes): (Vec<bool>, Vec<bool>) = xs.iter().map(occurences).unzip();
    let two = twos.iter().filter(|elt| **elt).count();
    let three = threes.iter().filter(|elt| **elt).count();
    two * three
}

fn part_b(_xs: &Vec<String>) -> i32 {
    unimplemented!()
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
    assert_eq!(true, true);
}
