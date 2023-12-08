use std::fs::File;
use std::io::{self, BufRead};


struct words {
    numChar: String,
    numInt: i32,
}


fn main() -> io::Result<()> {
    let path = "input.txt";

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut sum = 0;




    for line in reader.lines() {
        let line = line?;
        println!("{:?}", line);

        let mut left : i32 = -1;
        let mut right : i32 = -1;

        for i in line.chars() {
            if i.is_digit(10){
                if left == -1{
                    left = i as i32 - '0' as i32;
                    right = i as i32 - '0' as i32;
                } else {
                    right = i as i32 - '0' as i32;
                }
            }
        }
        println!("found: {:?},{:?}", left, right);
        let totals = left * 10 + right;
        println!("{:?}", totals);
        sum += totals;
    }
    println!("{:?}", sum);
    Ok(())

}
