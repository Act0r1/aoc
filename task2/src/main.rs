use std::collections::HashMap;
use std::{fs::File, io::Read, str::Chars};
// there a 9 ways to play this game like:
// A X 3 + 0 = 0, 3 for Scissors
// A Y 3 + 1 = 4,  3 for draw and 1 for Rock
// A Z 6 + 2 = 8, 6 for win and 2 for Papers 
// and also for B and C

fn main() {
    let mut h = HashMap::new();
    h.insert(["A", "X"], 3);
    h.insert(["A", "Y"], 4);
    h.insert(["A", "Z"], 8);
    h.insert(["B", "X"], 1);
    h.insert(["B", "Y"], 5);
    h.insert(["B", "Z"], 9);
    h.insert(["C", "X"], 2);
    h.insert(["C", "Y"], 6);
    h.insert(["C", "Z"], 7);

    let mut s = String::new();
    let mut f = File::open("inp.txt").unwrap();
    let mut sum = 0;
    f.read_to_string(&mut s).unwrap();
    for line in s.lines() {
        sum += handle_rps(&mut line.chars(), h.clone())
    }
    println!("{:?}", sum);
}

fn handle_rps(inp: &mut Chars, ans: HashMap<[&str; 2], i32>) -> i32 {
    let opponent = inp.nth(0).unwrap().to_string();
    let my = inp.nth_back(0).unwrap().to_string();
    let op_score = ans.get(&[&*opponent, &*my]).unwrap();
    *op_score
}
