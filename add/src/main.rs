use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("add/src/1.txt") {

        let mut  str_list =vec![];
        lines.for_each(|line| {
            if let Ok(line1) = line {
                str_list.push(line1);
            }
        });

        let mut sum = 0.0;
        let cnt = str_list.len();
        for item in str_list {
           let c = item.parse::<f64>().unwrap();
            sum += c;
        }
        println!("{:?}", sum/(cnt as f64))
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}