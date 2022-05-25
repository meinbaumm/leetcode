// https://leetcode.com/problems/largest-3-same-digit-number-in-string/
// 2264. Largest 3-Same-Digit Number in String


fn largest_good_integer(num: String) -> String {
    num.chars()
        .zip(num.chars().skip(1))
        .zip(num.chars().skip(2))
        .filter_map(|((a, b), c)| {
            if a == b && b == c {
                Some(format!("{}{}{}", a, b, c))
            } else {
                None
            }
        })
        .max()
        .unwrap_or_default()
}

fn main() {
    let num = "6777133339".to_string();
    println!("{}", largest_good_integer(num));
}
