fn main() {
    let a: Vec<i64> = _readline();
    println!("{:#?}", a);
}
fn _readline<T>() -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
        .split_whitespace()
        .filter_map(|word| word.parse().ok())
        .collect()
}