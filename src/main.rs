use clap::{command, arg};
fn main() {
    let matches = command!()
        .about("A simple CLI tool for converting 15 digit Salesforce Ids to 18 digits")
        .version("0.1.0")
        .arg(arg!([ids] ... "The 15 digit Salesforce Ids to convert"))
        .get_matches();

    if let Some(ids) = matches.get_many::<String>("ids") {
        println!("{:>15} | 18 Digit Ids\n{}|{}", "15 Digit Ids", "-".repeat(16), "-".repeat(19));
        for id in ids {
            if !is_valid_id(&id) {
                eprintln!("{} is not a valid Salesforce Id", id);
                continue;
            }
            if id.len() == 15 {
                println!("{} | {}", id, convert_id(&id));
            } else {
                println!("{} | {}", convert_id(&id), id);
            }
        }
    }
}

fn is_valid_id(id: &str) -> bool {
    id.len() == 15 || id.len() == 18 && id.chars().all(|c| c.is_ascii_alphanumeric())
}

fn convert_id(id: &str) -> String {
      match id.len() {
        15 => {
            let mut suffix = String::new();
            for i in 0..3 {
                let mut loop_val = 0;
                for j in 0..5 {
                    let current = id.chars().nth(i * 5 + j).unwrap();
                    if current >= 'A' && current <= 'Z' {
                        loop_val += 1 << j;
                    }
                }
                suffix.push("ABCDEFGHIJKLMNOPQRSTUVWXYZ012345".chars().nth(loop_val).unwrap());
            }
            format!("{}{}", id, suffix)
        },
        18 => {
            let mut id = id.to_owned();
            id.truncate(15);
            id
        },
        _ => unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_id() {
        assert!(is_valid_id("001000000000001"));
        assert!(is_valid_id("001000000000001AAA"));
        assert!(!is_valid_id("001000000000001AA"));
        assert!(!is_valid_id("001000000000001A!"));
        assert!(!is_valid_id("001000000000001 "));
    }

    #[test]
    fn test_convert_id() {
        assert_eq!(convert_id("001000000000001"), "001000000000001AAA");
        assert_eq!(convert_id("001000000000001AAA"), "001000000000001");
    }
}