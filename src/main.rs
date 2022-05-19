use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, io};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut tlc = 0;
    let mut twc = 0;
    let mut tbc = 0;

    for arg in args.iter() {
        let f = File::open(arg)?;
        let reader = BufReader::new(f);

        let mut lc = 0;
        let mut wc = 0;
        let mut bc = 0;
        for line in reader.lines() {
            match line {
                Ok(l) => {
                    lc += 1;
                    wc += l.split_whitespace().count();
                    bc += l.len() + 1;
                }
                Err(_) => {}
            }
        }

        tlc += lc;
        twc += wc;
        tbc += bc;

        println!("{:>8} {:>7} {:>7} {:>7}", lc, wc, bc, arg);
    }

    if args.len() > 1 {
        println!("{:>8} {:>7} {:>7} total", tlc, twc, tbc);
    }

    Ok(())
}
