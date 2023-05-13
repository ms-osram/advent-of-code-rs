use std::io::Read as _;

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .lock()
        .read_to_string(&mut buffer)
        .expect("Unable to read from stdin");

    let mut wrapping_paper = 0;
    let mut ribbon = 0;
    for line in buffer.split_whitespace() {
        
        let mut sides: Vec<u32> = line
        .split("x")
        .map(|s| s.parse().unwrap())
        .collect();

        sides.sort_unstable(); // to do: consult gen

        let l = sides[0]; 
        let w = sides[1];
        let h = sides[2];
        wrapping_paper += 2*l*w + 2*w*h + 2*h*l + l*w;
        ribbon += 2*l + 2*w + (l*w*h)

        //println!("{:?}", sides);
        
    }
    println!("The total area of wrapping paper is {} square feet", wrapping_paper);
    println!("The total length of ribbon is {} feet", ribbon)
}
