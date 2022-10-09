fn main() {
    let mut printed = 0;

    for i in 1..101 {
        if i % 3 == 0 {
            print!("Fizz");
            printed = 1;
        }

        if i % 5 == 0 {
            print!("Buzz");
            printed = 1;
        }

        if printed == 0 {
            println!("{}", i);
        }

        println!();

        printed = 0;
    }
}
