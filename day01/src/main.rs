use std::collections::HashSet;

fn main() {
    let xs: Vec<i32> = include_str!("data.txt")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();

    println!("part a sum: {}", part_a(&xs));
    println!("part b repeated sum: {}", part_b(&xs));
}

fn part_a(xs: &Vec<i32>) -> i32 {
    xs.iter().sum()
}

fn part_b(xs: &Vec<i32>) -> i32 {
    let initial = xs.iter().take(2).sum();
    let mut sums = HashSet::new();
    sums.insert(initial);
    let it = xs.iter().skip(2).chain(xs.iter().cycle());
    let val = it
        .scan(initial, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .find(|&v| !sums.insert(v));
    val.unwrap()
}

#[test]
fn test_part_a() {
    assert_eq!(part_a(&vec![1, 1, 1]), 3);
    assert_eq!(part_a(&vec![1, 1, -2]), 0);
    assert_eq!(part_a(&vec![-1, -2, -3]), -6);
}

#[test]
fn test_part_b() {
    assert_eq!(part_b(&vec![1, -1]), 0);
    assert_eq!(part_b(&vec![3, 3, 4, -2, -4]), 10);
    assert_eq!(part_b(&vec![-6, 3, 8, 5, -6]), 5);
    assert_eq!(part_b(&vec![7, 7, -2, -7, -4]), 14);
}
