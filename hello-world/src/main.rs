fn main()
{
    let i: i32 = 655;

    let b: bool = true;

    let c: char = 'A';

    let f: f32 = 5.2;

    let d: f64 = 3.2;

    let p: &i32 = &i;

    let s: &str = "Hello";

    let arr: [i32; 4] = [1, 5, 6, 8];

    let t: (char, bool, i32) = ('c', true, 35);

    let mut vec: Vec<char> = Vec::new();

    vec.push('r');

    println!("Hello, {:?} world!", arr);

}
