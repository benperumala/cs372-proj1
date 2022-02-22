use std::collections::HashMap;  // Import the standard library hash map

// Define constants for fib(0) and fib(1)
const FIB_ZERO: u64 = 1;
const FIB_ONE: u64 = 1;

/// A simple recursive fibonacci implementation
fn fib(n: u64) -> u64 {
    // The if counts as a single expression. No semi-colon = return
    // Each statement with the if/else if/else is a return
    // (which the if statement yields which then the function returns)
    if n == 0 {
        FIB_ZERO
    } else if n == 1 {
        FIB_ONE
    } else {
        fib(n- 1) + fib(n-2)
    }
}

/// A dynamic programming version of fibonacci
/// (a fancy way of saying previously computed values are saved)
fn fib_dyn(n: u64, map: &mut HashMap<u64, u64>) -> u64 {
    // This is the last statement of the function so
    // the return from within the match is the return
    // of the entire function
    match n {
        0 | 1 => 1,  // If n is 0 or 1, return 1
        n => {
            if map.contains_key(&n) {
                // .get() returns Some() - an object which can either have our result or be None.
                // Because we know the map contains the item (we checked that in the above line)
                // we know for sure it's not None. So we can use `.unwrap()` to get the
                // value from the Some() object (Some(integer).unwrap() -> integer).
                // (Calling .unwrap() on None, it produces a panic/exception so be careful of that)
                *map.get(&n).unwrap()
            } else {
                let val = fib_dyn(n-1, map) + fib_dyn(n-2, map);
                map.insert(n, val);
                return val;
            }
        }
    }
}

fn main() {
    let mut map = HashMap::new();

    println!("Dynamic Programming version:");
    //      [1, 40)
    for i in 1..40 {
        println!("{}: {}", i, fib_dyn(i, &mut map));
    }

    println!("\nRegular Recursive version:");
    //      [1, 40)
    for i in 1..40 {
        println!("{}: {}", i, fib(i));
    }
}
