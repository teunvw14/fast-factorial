use std::env;
use std::thread;
use std::sync::mpsc;
use std::time::Instant;

use num_cpus;
use num_bigint::BigUint;


fn factorial_partial(n: u128, k: u128) -> BigUint {
    // Calculate part of the factorial, (n-k)! to be exact.
    if n == 1 || n == 0 {
        return BigUint::from(1 as u128);
    }
    let mut result = BigUint::from(n);
    if k >= n { 
        panic!("k has to be smaller than n, got n: {}, k: {}", n, k);
    }
    for i in k..n {
        result *= i;
    }
    result
}

fn factorial_multithreaded(n: u128) -> BigUint {
    let mut result = BigUint::from(1 as u8);
    let max_threads = num_cpus::get();
    let mut handles = vec![];
    // Create a mspc channel to send and receive the results of
    // the calculations.
    let (tx, rx) = mpsc::channel();
    // Calculate how many multiplications each thread should do to evenly 
    // split computation across available threads.
    let mul_per_thread = (n as f64 / max_threads as f64).ceil() as u128;
    let mut m = n;
    let mut k;
    while m > 1 {
        if m <= mul_per_thread {
            k = 1;
        } else {
            k = m - mul_per_thread;
        }
        let producer = tx.clone();
        let handle = thread::spawn(move || {
            let intermediate_result = factorial_partial(m, k);
            producer.send(intermediate_result).unwrap();
        });
        handles.push(handle);
        if m > mul_per_thread { 
            m -= mul_per_thread + 1; 
        } else {
            break;
        }
    }
    drop(tx); // force tx to go out of scope
    for ires in rx {
        result *= ires;
    }
    result
}

fn main() {
    let help_message_usage = "Usage: factorial [number]";
    let error_message_num_missing = format!("
            Please enter a number to calculate the factorial of.
            {}
        ", help_message_usage);
    let num_string = env::args().nth(1).expect(&error_message_num_missing);

    let error_message_num_parse = format!("
            Something went wrong parsing the number.
            Maximum input is: {}
            Usage: {}
        ", u128::MAX, help_message_usage);
    let num: u128 = u128::from_str_radix(&num_string, 10).expect(&error_message_num_parse);
    
    // Calculate factorial and display result:
    println!("Calculating {}! with {} threads...", num, num_cpus::get());
    let start = Instant::now();
    let result = factorial_multithreaded(num);
    let elapsed = start.elapsed().as_millis();
    println!("{}! = {}", num, result);
    println!("Done, took {}ms.", elapsed);
}
