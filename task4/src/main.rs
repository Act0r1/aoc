use std::fs::File;
use std::io::Read;
fn main() {
    let mut s = String::new();
    let mut f = File::open("inp.txt").unwrap();
    f.read_to_string(&mut s).unwrap();
    let d: Vec<&str> = s.lines().into_iter().collect();
    let first:Vec<_> = d.iter().map(|c| c.split(",").next().unwrap()).collect();
    let second:Vec<_> = d.iter().map(|c| c.split(",").last().unwrap()).collect();
}
