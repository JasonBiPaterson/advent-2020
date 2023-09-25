use std::fs;

fn from_snafu_head(c: char, len: usize) -> u64 {
    let n = match c {
        '1' => 1,
        '2' => 2,
        _ => unreachable!(),
    };
    let mut ans = 0;
    for i in 0_u32..((len as u32) - 1) {
        ans += 2 * 5_u64.pow(i)
    }
    ans += 1;
    ans += (n - 1) * 5_u64.pow((len as u32) - 1);
    ans
}

fn from_snafu(snafu: &str) -> u64 {
    let chars = snafu.chars().collect::<Vec<char>>();
    let snafu_len = chars.len();
    let mut ans: u64 = 0;
    for i in 0..((snafu_len) - 1) {
        let c = chars[snafu_len - i - 1];

        let n = match c {
            '=' => 0,
            '-' => 1,
            '0' => 2,
            '1' => 3,
            '2' => 4,
            _ => unreachable!(),
        };
        ans += 5_u64.pow(i as u32) * n;
    }
    ans += from_snafu_head(chars[0], snafu_len);
    ans
}

fn sum_snafu(input: &str) -> u64 {
    let snafus = input.lines().collect::<Vec<&str>>();
    let mut ans = 0;
    for snafu in snafus {
        ans += from_snafu(snafu);
    }
    ans
}

fn to_snafu(n: u64) -> String {
    let mut ans = vec![];
    for i in 1.. {
        if n <= from_snafu_head('1', i) {
            ans = vec!['='; i - 1];
            ans[0] = '2';
            break;
        } else if n <= from_snafu_head('2', i) {
            ans = vec!['='; i];
            ans[0] = '1';
            break;
        }
    }

    for i in 1..ans.len() {
        for c in ['-', '0', '1', '2'] {
            let mut tmp = ans.clone();
            tmp[i] = c;
            let s: String = tmp.iter().collect();
            if from_snafu(&s) > n {
                break;
            } else {
                ans = tmp;
            }
        }
    }
    ans.iter().collect()
}

fn main() {
    let input: String = fs::read_to_string("input/day25").expect("Failed to read input");

    println!("Part 1: {}", to_snafu(sum_snafu(&input)));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn test_part1() {
        let snafu_sum = sum_snafu(TEST_INPUT);
        assert_eq!(snafu_sum, 4890);
        assert_eq!(to_snafu(snafu_sum), "2=-1=0");
    }
}
