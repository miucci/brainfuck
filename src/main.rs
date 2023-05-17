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
            ',' => {
                data[dp] = std::io::stdin()
                    .bytes()
                    .next()
                    .unwrap_or(Ok(0))
                    .unwrap_or(0)
            }
            '[' => {
                if data[dp] == 0 {
                    ip = find_matching_bracet(&code, ip)
                        .unwrap_or_else(|| exit_error("unmatched bracets"))
                        + 1;
                    continue;
                }
            }
            ']' => {
                if data[dp] != 0 {
                    ip = find_matching_bracet(&code, ip)
                        .unwrap_or_else(|| exit_error("unmatched bracets"))
                        + 1;
                    continue;
                }
            }
            _ => {}
        }
        ip += 1;
        if ip >= code.len() {
            break;
        }
    }
}

fn exit_error(msg: &str) -> ! {
    eprintln!("ERROR: {msg}");
    std::process::exit(1);
}

fn parse_args() -> Vec<char> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        exit_error("invalid number of arguments");
    }
    std::fs::read_to_string(&args[1])
        .unwrap_or_else(|_| exit_error("could no open file"))
        .chars()
        .collect()
}

fn find_matching_bracet(code: &[char], index: usize) -> Option<usize> {
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
                    break Some(index + i);
                }
                i += 1;
                if index + i >= code.len() {
                    break None;
                }
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
                    break Some(index - i);
                }
                if index - i == 0 {
                    break None;
                }
                i += 1;
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_matching() {
        let code: Vec<char> = "+++[++<-[]]".chars().collect();
        assert_eq!(Some(3), find_matching_bracet(&code, 10));
        assert_eq!(Some(10), find_matching_bracet(&code, 3));
        assert_eq!(Some(8), find_matching_bracet(&code, 9));
    }
}
