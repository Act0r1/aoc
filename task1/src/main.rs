use std::{
    fs::File,
    io::{BufRead, BufReader, Read},
    println,
};
fn main() {
    let mut s = String::new();
    let mut file = File::open("inp.txt").unwrap();
    let mut sum = 0;
    let mut ans: Vec<i32> = Vec::new();
    file.read_to_string(&mut s).unwrap();
    for line in s.lines() {
        if "" == line {
            ans.push(sum);
            sum = 0;
        }
        sum += line.trim().parse::<i32>().unwrap_or_default();
    }
    ans.sort();
    ans.reverse();
    let a1 = ans.iter().nth(0).unwrap();
    let a2 = ans.iter().nth(1).unwrap();
    let a3 = ans.iter().nth(2).unwrap();
    println!("{:?}", a1+a2+a3);

}
