use std::str::FromStr;

fn main() {
    let args = std::env::args().collect();
    count(args)
}

fn count(args: Vec<String>) {
    let x = args
        .get(1)
        .map(|s: &String| i32::from_str(s))
        .unwrap_or(Ok(0))
        .unwrap_or_else(|err| {
            println!("Error while parsing: {}", err);
            0
        });

    for i in x..10 {
        println!("{i}")
    }
}

#[cfg(test)]
mod test {

    use super::count;
    #[test]
    fn test_my_program() {
        let test_vec = vec![String::from("foo"), String::from("1")];
        count(test_vec)
    }

    #[test]
    fn test_my_program_42() {
        let test_vec = vec![String::from("foo"), String::from("42")];
        count(test_vec)
    }
}
