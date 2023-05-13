use std::io::Read as _;
use md5::compute;

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .lock()
        .read_to_string(&mut buffer)
        .expect("Unable to read from stdin");


    let mut number = 0;
    let mut answer = None;

    for line in buffer.split_whitespace() {

        while answer == None {
            let string = format!("{}{}", line, number);
            let hash = compute(string);

            //println!("{:?}", *hash);
            //println!("{:?}\n", hash);
            let check_hash = format!("{:x}", hash);

            if hash[0] == 0 {
                println!("let's go!");
            }

            // 0 0 0 0 x x x x <=
            //         8 4 2 1
            // \_____/ \_____/
            // consider
            //

            if &check_hash[0..5] == "000000" {
                answer = Some(number);
                break;
            } else {
                number += 1;
            }
        }
    println! ("The answer is {:?}", answer.unwrap());  
    }
}
