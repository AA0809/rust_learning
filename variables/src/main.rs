fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let y = 7i64;
    let z = 8i32;

    let chr: char = 'x';
    println!("Character is {chr}");

    let tup: (u8, i32, f64) = (1,2,3.0);
    println!("tup.1 is {}", tup.1);
    let tup = (1,2,3);
    println!("shadowed tup.1 is {}", tup.1);

    let arr = [1,2,3,4,5];
    println!("array[1] is {}", arr.len());
}