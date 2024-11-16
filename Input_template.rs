use std::io::{self, BufRead};

fn single_line_single_input() {
    let stdin=io::stdin();
    let mut iterator=stdin.lock().lines();
    // takes in the testcase
    let t:i32=iterator.next().unwrap().unwrap().trim().parse().unwrap();
}
