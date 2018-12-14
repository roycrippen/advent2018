fn main() {
    let s: String = include_str!("data.txt").to_string();
    // let s: String = "dabAcCaCBAcCcaDA".to_string();

    println!("part a: {}", part_a(&s)); // 10638
    println!("part b: {}", part_b(&s)); // 4944
}

fn reacts(a: u8, b: u8) -> bool {
    if a < b {
        b - a == 32
    } else {
        a - b == 32
    }
}

fn react(s: &str) -> String {
    let mut res: Vec<char> = Vec::new();
    let xs = s.as_bytes().to_vec();
    let mut i = 0;
    while i < xs.len() - 1 {
        if reacts(xs[i], xs[i + 1]) {
            i += 1;
        } else {
            res.push(char::from(xs[i]));
            react_last_two(&mut res)
        }
        i += 1;
    }
    if i == xs.len() - 1 {
        res.push(char::from(xs[i]));
        react_last_two(&mut res)
    }
    res.iter().collect()
}

fn react_last_two(xs: &mut Vec<char>) {
    if xs.len() > 1 {
        let a = xs.get(xs.len() - 1).unwrap();
        let b = xs.get(xs.len() - 2).unwrap();
        if reacts(*a as u8, *b as u8) {
            xs.pop();
            xs.pop();
        }
    }
}

fn part_a(s: &String) -> usize {
    let res = react(&s);
    res.len()
}

fn part_b(s: &String) -> usize {
    let filter_char = |exclude: u8| {
        let exclude_char = exclude as char;
        s.chars()
            .filter(|c| *c != exclude_char && *c != exclude_char.to_ascii_uppercase())
            .collect()
    };

    let xs: Vec<usize> = (b'a'..b'z' + 1).map(|c| part_a(&filter_char(c))).collect();

    *xs.iter().min_by(|a, b| a.cmp(b)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_react() {
        let s: String = "aA".to_string();
        let res = react(&s);
        println!("{:?}", &res);
        assert_eq!(res, "".to_string());

        let s: String = "abBA".to_string();
        let res = react(&s);
        println!("{:?}", &res);
        assert_eq!(res, "".to_string());

        let s: String = "aabAAB".to_string();
        let res = react(&s);
        println!("{:?}", &res);
        assert_eq!(res, "aabAAB".to_string());
    }

    #[test]
    fn test_part_a() {
        let s: String = "dabAcCaCBAcCcaDA".to_string();
        assert_eq!(part_a(&s), 10);
        let s: String = "dbcCCBcCcD".to_string();
        assert_eq!(part_a(&s), 6);
        let s: String = "daAcCaCAcCcaDA".to_string();
        assert_eq!(part_a(&s), 8);
        let s: String = "dabAaBAaDA".to_string();
        assert_eq!(part_a(&s), 4);
        let s: String = "abAcCaCBAcCcaA".to_string();
        assert_eq!(part_a(&s), 6);
    }

    #[test]
    fn test_part_b() {
        let s: String = "dabAcCaCBAcCcaDA".to_string();
        assert_eq!(part_b(&s), 4);
    }
}
