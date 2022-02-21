use std::collections::HashMap;

const FIB_ZERO: u64 = 1;
const FIB_ONE: u64 = 1;

/// A simple recursive fibonacci implementation
fn fib(n: u64) -> u64 {
    // The if counts as a single expression. No semi-colon = return
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
    match n {
        0 | 1 => 1,  // If n is 0 or 1, return 1
        n => {
            if map.contains_key(&n) {
                // .get() returns Some() which can be None.
                // Because we know the map contains the item (above line)
                // we know for sure it's not None. So we can use `.unwrap()` to get the
                // value from the Some() object
                // (If we call .unwrap() on None, it produces a panic/exception)
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
    //          [1, 40)
    for i in 1..40 {
        println!("{}: {}", i, fib_dyn(i, &mut map));
    }

    println!("\nRegular Recursive version:");
    //           [1, 40)
    for i in 1..40 {
        println!("{}: {}", i, fib(i));
    }
}
