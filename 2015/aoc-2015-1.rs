use std::io::Read as _;

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .lock()
        .read_to_string(&mut buffer)
        .expect("Unable to read from stdin");

    
    for line in buffer.split_whitespace() {
        let mut count = 0;
        let mut pos = None;
        for (idx, ch) in line.chars().enumerate() {
            match ch {
                '(' => count += 1,
                ')' => count -= 1,
                '\n' => break,
                _ => unreachable!(),
            }

            if count == -1 {
                if pos.is_none() {
                    pos = Some(idx + 1);
                }
            }

        }
        println! ("Santa reached basement at position {}", pos.unwrap());
        println! ("The total count is {}!", count);
    }
}