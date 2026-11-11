// Unsigned integer only accept zeros and positive values.
// Signed integer accepts positive and negative values.

fn main() {
    // 1. Integer
    let age: u32 = 23;
    let temperature: i32 = -10;

    // 2. Floating Point
    let price: f64 = 99.99;      //  f64 is the default 64-bit floating-point type.
    let rating: f32 = 4.5;

    // 3. Boolean
    let is_active: bool = true;

    // 4. Character
    let initial: char = 'A';

    // 5. Tuple
    let user: (&str, u32, bool) = ("Aditya", 24, true);

    let (name, user_age, active) = user;

    // 6. Array
    let numbers: [i32; 5] = [10, 20, 30, 40, 50];

    let first = numbers[0];
    let second = numbers[1];

    // 7. Vector
    let mut numbers = vec![10, 20, 30];

    numbers.push(40);

    
    println!("Age: {age}");
    println!("Temperature: {temperature}");
    println!("Price: {price}");
    println!("Rating: {rating}");
    println!("Active: {is_active}");
    println!("Initial: {initial}");
    
    println!("Name: {name}");
    println!("User Age: {user_age}");
    println!("User Active: {active}");
    
    println!("First Number: {first}");
    println!("Second Number: {second}");
    println!("{:?}", numbers);
}
