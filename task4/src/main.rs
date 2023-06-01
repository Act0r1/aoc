use std::fs;
fn main() {
    let s = fs::read_to_string("inp.txt").unwrap();
    let lines: Vec<&str> = s.lines().into_iter().collect();
    let first: Vec<_> = lines.iter().map(|c| c.split(",").next().unwrap()).collect();
    let second: Vec<_> = lines.iter().map(|c| c.split(",").last().unwrap()).collect();
    let mut sum = 0;
    let len = first.len() - 1;
    for x in 0..=len {
        let (a, b) = parse(first[x]);
        let (c, d) = parse(second[x]);
        let cond1 = c >= a && b >= d || a >= c && d >= b;
        match cond1 {
            true => {
                sum += 1;
            }
            _ => {}
        }
    }
    println!("{:?}", sum);
}

fn parse(src: &str) -> (i32, i32) {
    let all_nums: Vec<_> = src.split("-").collect();
    (
        all_nums[0].parse::<i32>().unwrap(),
        all_nums[1].parse::<i32>().unwrap(),
    )
}
