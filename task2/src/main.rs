use std::collections::HashMap;
use std::{fs::File, io::Read, str::Chars};
// there a 9 ways to play this game like:
// A X 3 + 1 = 4, 3 for draw and 1 for X or ROCK
// A Y 6 + 2 = 8, 6 for win and 2 for Paper
// A Z 3 + 0 = 3, 0 for lost and 3 for Scissors


fn main() {
    let mut h = HashMap::new();
    h.insert("A", 1);
    h.insert("B", 2);
    h.insert("C", 3);
    h.insert("X", 1);
    h.insert("Y", 2);
    h.insert("Z", 3);
    
    let mut s = String::new();
    let mut f = File::open("inp.txt").unwrap();
    f.read_to_string(&mut s).unwrap();
    for line in s.lines() {
        println!("{:?}",handle_rps(&mut line.chars(), h));
        break;
    }
}

fn handle_rps(inp: &mut Chars, ans: HashMap<&str,i32>)  {
    let opponent = inp.nth(0).unwrap().to_string();
    let my = inp.nth_back(0).unwrap().to_string();
    let op_score = ans.get(&*opponent).unwrap();
    let my_score = ans.get(&*my).unwrap();
}
