// Although variables are immutable by default, you can make them mutable by adding mut in front of the variable name
// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }


// Shadow

fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("Inner x: {x}");
    }

    println!("Outer x: {x}");
}