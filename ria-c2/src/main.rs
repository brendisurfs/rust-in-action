use num::complex::Complex;
fn main() {
    let a = 32;
    let b: i16 = 32;

    let b_two: i32 = b.try_into().unwrap();
    assert_eq!(a, b_two, "testing if a and b_two are the same");

    // complex numbers
    /*
      @dev: using complex numbers
    */

    let comp_a = Complex { re: 2.1, im: -1.2 };
    let comp_b = Complex::new(11.3, 22.2);
    let res = comp_a + comp_b;
    println!("complex addition: {} {}", res.re, res.im);

    // Loop basics

    let my_data = vec![1, 2, 3, 4, 5, 6];

    // read-only.
    // equivalent of: for item in my_data.iter()
    for item in &my_data {
        println!(" read-only borrow: {}", item + 1)
    }

    // ownership
    for item in my_data {
        println!("owned: {item}")
    }

    // read-write
    // for item in &mut my_data { // note: the array/vec/whatever must be mut too.
    //     println!("rw: {}", item)
    // }

    // 2.4.2 Continue
    for n in 0..10 {
        if n % 2 == 0 {
            println!(" continue example: {n}");
            continue;
        }
    }

    // 2.4.3 While loop condition
    /*let mut samples = vec![];

    while samples.len() < 10 {
        let sample = take_sample(); // made up sampling function
        if is_outlier(sample) {
            continue;
        }
        samples.push(sample);
    }
    */

    // break labels
    'outer: for x in 0.. {
        for y in 0.. {
            for z in 0.. {
                if x + y + z > 1000 {
                    println!(" break labels: {}", x + y + z);
                    break 'outer;
                }
            }
        }
    }

    // 2.4.7 Match
    // remember: rust is an expression based language.
    println!("\n 2.4.7 Match section");

    fn is_even(num: i32) -> bool {
        num % 2 == 0
    }

    let n = 123456;
    let description = match is_even(n) {
        true => "even",
        false => "odd",
    };

    println!("{}", description);

    let my_match = 10..50;

    'outside: for item in my_match {
        match item {
            10 => println!("number is between 10 and 20"),
            20..=40 => {
                println!("number is between 20 and 40");
                break 'outside;
            }
            _ => println!("I got lazy and just know there arent many nums left"),
        }
    }

    println!("\n another match example: \n ");

    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    for n in haystack {
        let res = match n {
            42 | 132 => "hit",
            _ => "miss",
        };
        if res == "hit" {
            println!("{n}, {res}")
        }
    }

    let searc_word = "picture";

    let quote = "Every face, every shop, bedroom window, public-house, and
    dark square is a picture feverishly turned--in search of what?
    It is the same with books. What do we seek through millions of pages?";

    for (i, line) in quote.lines().enumerate() {
        if line.contains(searc_word) {
            let line_num = i + 1;
            println!("{line_num}: {line}");
        }
    }
}
