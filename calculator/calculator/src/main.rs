use std::io;

fn main() {
    println!("Enter a calculation mode");

    let mut mode = String::new();
    let mut is_running: bool = true;
    let mut number_one: u32;
    let mut number_two: u32;
    let mut readed_number = String::new();

    loop {
        if !is_running {
            break;
        }
        readed_number.clear();
        mode.clear();

        io::stdin()
                .read_line(&mut mode)
                .expect("Failed to read line");

        println!("selected mode : {}", mode);


        io::stdin()
                .read_line(&mut readed_number)
                .expect("Failed to read line");

        number_one = readed_number.trim().parse().expect("this is not a number");

        readed_number.clear();
        io::stdin()
                .read_line(&mut readed_number)
                .expect("Failed to read line");

        number_two = readed_number.trim().parse().expect("this is not a number");


        let result:u32 = match mode.as_str().trim() {
                "+" => add_number(number_one, number_two),
                "-" => sub_number(number_one, number_two),
                "q" => {
                    is_running = false;
                    0
                    },
                _ => {
                    println!("mode not found");
                    0
                },
            };
        println!("result : {}", result);
    }
}

fn add_number(a:u32, b:u32) -> u32
{
    return a + b;
}

fn sub_number(a:u32, b:u32) -> u32
{
    return a - b;
}
