struct Solution;
impl Solution {
    pub fn num_sub(s: String) -> i32 {
        const MOD: i64 = 1e9 as i64 + 7;
        let combine_num = |x: i64| (x * (x + 1) / 2) % MOD;
        let mut ans = 0;
        let mut length = 0;
        for c in s.chars() {
            match c {
                '0' => {
                    ans = (ans + combine_num(length)) % MOD;
                    length = 0;
                }
                '1' => {
                    length += 1;
                }
                _ => panic!(),
            }
        }
        ans = (ans + combine_num(length)) % MOD;
        ans as i32
    }
}
fn main() {
    // Test case
    let test_input = String::from("0110111");
    let result = Solution::num_sub(test_input.clone());
    println!("Input: {}", test_input);
    println!("Output: {}", result);

    // Additional test cases
    let test_cases = vec![
        ("111111", 21),
        ("0110111", 9),
        ("101", 2),
        ("111", 6),
        ("1", 1),
    ];

    for (input, expected) in test_cases {
        let result = Solution::num_sub(input.to_string());
        println!(
            "Input: '{}', Expected: {}, Got: {}, {}",
            input,
            expected,
            result,
            if result == expected { "✓" } else { "✗" }
        );
    }
}
