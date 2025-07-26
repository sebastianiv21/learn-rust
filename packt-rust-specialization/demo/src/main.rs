pub fn factorial(n: u32) -> u32 {
    if n == 0 {
        return 1;
    }
    let mut res = 1;
    for x in 1..=n {
        dbg!(x);
        dbg!(res);
        res *= x;
    }
    dbg!(res);
    res
}

fn main() {
    let result = factorial(5);
    dbg!(result);
}
