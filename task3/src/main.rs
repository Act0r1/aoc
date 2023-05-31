use std::char;
use std::fs::File;
use std::io::Read;
fn main() {
    let mut s = String::new();
    let mut f = File::open("inp.txt").unwrap();
    f.read_to_string(&mut s).unwrap();
    let lower_case: Vec<_> = (10..36)
        .map(|i| char::from_digit(i, 36).unwrap())
        .zip(1..=26)
        .collect();

    let upper_case: Vec<_> = lower_case
        .iter()
        .map(|(c, _)| c.to_uppercase().next().unwrap())
        .zip(27..=52)
        .collect();
    let mut sum = 0;
    let v: Vec<_> = s.lines().into_iter().collect();
    let mut res = v.chunks(3);
    while let Some(s) = res.next() {
        let first_hash: Vec<char> = s.first().unwrap().chars().into_iter().collect();
        let second_hash: Vec<char> = s[1].chars().into_iter().collect();
        let third_hash: Vec<char> = s.last().unwrap().chars().into_iter().collect();
        let contains_char = first_hash
            .iter()
            .find(|c| second_hash.contains(c) && third_hash.contains(c))
            .unwrap();
        match contains_char.is_uppercase() {
            true => {
                let (_, num) = upper_case.iter().find(|(c, _)| c == contains_char).unwrap();
                sum += num;
            }
            false => {
                let (_, num) = lower_case.iter().find(|(c, _)| c == contains_char).unwrap();
                sum += num;
            }
        }
    }
    println!("{:?}", sum);

    // for x in s.lines().collect {
    //     let line = x.to_string();
    //     // println!("{:?}", line.split_at(x.len() / 2));
    //     let (first, second) = line.split_at(x.len() / 2);
    //     let first_hash: Vec<char> = first.chars().into_iter().collect();
    //     let second_hash: Vec<char> = second.chars().into_iter().collect();
    //     let both_contains_char = first_hash.iter().find(|c| second_hash.contains(c) && !second_hash.contains(c)).unwrap();
    //     match both_contains_char.is_uppercase() {
    //         true => {
    //             let (_, num) = upper_case
    //                 .iter()
    //                 .find(|(c, _)| c == both_contains_char)
    //                 .unwrap();
    //             sum += num;
    //         }
    //         false => {
    //             let (_, num) = lower_case
    //                 .iter()
    //                 .find(|(c, _)| c == both_contains_char)
    //                 .unwrap();
    //             sum += num;
    //
    //         }
    //     }
    // }
    // println!("{:?}", sum);
}
