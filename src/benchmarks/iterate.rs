pub fn iterate(max: i32) -> i32 {
    let mut res = 0;
    for i in 0..max {
        res += i;
    };
    res
}