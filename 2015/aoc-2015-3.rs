use std::io::Read as _;
use std::collections::HashSet;

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .lock()
        .read_to_string(&mut buffer)
        .expect("Unable to read from stdin");

    let mut coords = HashSet::new();
    for line in buffer.split_whitespace() {
        let mut hor_position = 0;
        let mut ver_position = 0;
        let mut robo_hor_position = 0;
        let mut robo_ver_position = 0;
        for (idx, ch) in line.chars().enumerate() {
            let (cur_hor, cur_ver) = match idx % 2 {
                0 => (&mut hor_position, &mut ver_position),
                1 => (&mut robo_hor_position, &mut robo_ver_position),
                _ => unreachable!(),
            };

            match ch {
                '^' => *cur_ver += 1,
                'v' => *cur_ver -= 1,
                '<' => *cur_hor -= 1,
                '>' => *cur_hor += 1,
                '\n' => break,
                _ => unreachable!(),
            }

            coords.insert((*cur_hor, *cur_ver));

            /*
            if idx % 2 == 0 {
                match ch {
                    '^' => ver_position += 1,
                    'v' => ver_position -= 1,
                    '<' => hor_position -= 1,
                    '>' => hor_position += 1,
                    '\n' => break,
                    _ => unreachable!(),
                };   
                coords.insert((hor_position, ver_position));
            } else {
                match ch {
                    '^' => robo_ver_position += 1,
                    'v' => robo_ver_position -= 1,
                    '<' => robo_hor_position -= 1,
                    '>' => robo_hor_position += 1,
                    '\n' => break,
                    _ => unreachable!(),
                };
                coords.insert((robo_hor_position, robo_ver_position));
            }
            */

            
            
            
            
            

        
    }
    println! ("The number of houses that received atleast one present is {}", coords.len());
}
}