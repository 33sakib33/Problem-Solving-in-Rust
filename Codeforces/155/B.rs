
use std::io::{self, BufRead};



 
fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let mut turns=1;
    let mut extras: Vec<(i32, i32)> = Vec::new();
    let mut ans=0;
    let n: usize = iterator.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..n {
        let line = iterator.next().unwrap().unwrap();
        let numbers: Vec<i32> = line
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Parse error"))
        .collect();
        if numbers[1]>0 {
            ans=ans+numbers[0];
            turns=turns+(numbers[1]-1);
        }
        else{
            extras.push((numbers[0],numbers[1]));
        }
    }
    extras.sort();
    extras.reverse();
    for x in &extras{
        if turns==0 {
            break;
        }
        
        ans=ans+x.0;
        turns-=1;
    }
    println!("{}",ans);
}
