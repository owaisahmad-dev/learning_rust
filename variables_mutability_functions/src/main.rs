const MAX_POINTS: u32 = 100_000; // constants can be declared in global scope

// You are climbing a staircase. It takes n steps to reach the top.
// Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
fn climbing_stairs(number_of_steps: u32) -> i32 {
    if number_of_steps == 1 {
        return 1;
    }
    if number_of_steps == 2 {
        return 2;
    }

    let mut next_step = 1;
    let mut next_two_steps = 2;

    let mut number_of_ways = 0;

    for _ in (0..number_of_steps - 2).rev() {
        number_of_ways = next_step + next_two_steps;
        next_step = next_two_steps;
        next_two_steps = number_of_ways;
    }

    return number_of_ways;
}

fn main() { 
    let mut x = 5; // mut is required to make a variable mutable
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    let y = 10; // immutable by default
    println!("The value of y is: {y}");
    let y = MAX_POINTS / x; // shadowing
    println!("The value of MAX_POINTS / x is: {y}");

    let possible_test_values = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    for i in possible_test_values.iter() {
        let res = climbing_stairs(*i);
        println!("The number of ways to climb {i} steps is: {res}");
    }    
}
