fn main() {
    let mut print_out = 10;
    let mut counter = 0;

    loop {
        println!("{print_out}");
        print_out += 1;
        if counter == 10 {
            break
        } else {
            counter += 1;
        }
    }

    println!("Fetttig")
}
