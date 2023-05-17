use std::io::Read;

fn main() {
    let code = parse_args();
    let mut data: [u8; 30_000] = [0; 30_000];
    let mut ip: usize = 0;
    let mut dp: usize = 0;
    loop {
        match code[ip] {
            '>' => dp += 1,
            '<' => dp -= 1,
            '+' => data[dp] = data[dp].wrapping_add(1),
            '-' => data[dp] = data[dp].wrapping_sub(1),
            '.' => print!("{}", data[dp] as char),
            ',' => data[dp] = std::io::stdin().bytes().next().unwrap().unwrap(),
            '[' => {
                if data[dp] == 0 {
                    ip = find_matching_bracet(&code, ip) + 1;
                    continue;
                }
            }
            ']' => {
                if data[dp] != 0 {
                    ip = find_matching_bracet(&code, ip) + 1;
                    continue;
                }
            }
            _ => {}
        }
        ip += 1;
    }
}

fn parse_args() -> Vec<char> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("invalid number of arguments");
        std::process::exit(1);
    }
    std::fs::read_to_string(&args[1]).unwrap().chars().collect()
}

fn find_matching_bracet(code: &[char], index: usize) -> usize {
    match code[index] {
        '[' => {
            let mut bracet_count = 1;
            let mut i = 1;
            loop {
                match code[index + i] {
                    '[' => bracet_count += 1,
                    ']' => bracet_count -= 1,
                    _ => {}
                }
                if bracet_count == 0 {
                    break index + i;
                }
                i += 1;
            }
        }
        ']' => {
            let mut bracet_count = 1;
            let mut i = 1;
            loop {
                match code[index - i] {
                    '[' => bracet_count -= 1,
                    ']' => bracet_count += 1,
                    _ => {}
                }
                if bracet_count == 0 {
                    break index - i;
                }
                i += 1;
            }
        }
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_matching() {
        let code: Vec<char> = "+++[++<-[]]".chars().collect();
        assert_eq!(3, find_matching_bracet(&code, 10));
        assert_eq!(10, find_matching_bracet(&code, 3));
        assert_eq!(8, find_matching_bracet(&code, 9));
    }
}
