// Name: Rust benchmark String vs SmartString and LTO
//
// Description: This little project tests 2 things. The first one is a benchmark
//              between the the creation of 1.000.000 (million) strings with a
//              normal Rust String type and a SmartString, I tested for a string
//              length lower or equal to 23 chars and for a string with longer
//              length. This size is important because SmartString has no Heap
//              allocation for small strings, if it's length is lower or equal to
//              23 bytes (ex: 23 ASCII chars). The difference in performance is
//              huge in that case!
//              And contrary what I have read in the SmartString documentation
//              for bigger string sizes the performance is the same. At least if
//              you do what I did, turn on the LTO - Link-Time Optimization flag
//              for longer strings, the processing times are also equal.
//              The second thing that this benchmark tests is the performance uplift
//              with and with out LTO - Link-Time Optimization flag compilation flag.
//              The compilation times for release build are a little longer, but the
//              performance uplift is huge! This without changing one line of code!
//              So is more performance for free :-)
//
// Author: Joao Nuno Carvalho
//
// Usage note: 
//      When creating SmartString's from &str instead of doing:
//
//      "blabla".to_string()
//
//      Do:
//      String::from("blabla")
//
// Date: 2021.11.10
//
// License: MIT Open Source License
//
// References:
//            Crate SmartString
//            https://crates.io/crates/smartstring
//
//            How to learn modern Rust - Compilation flags
//            https://github.com/joaocarvalhoopen/How_to_learn_modern_Rust#rust-optimization---compilation-modes-and-flags
//
//            The Rust Performance Book - Heap Allocations
//            https://nnethercote.github.io/perf-book/heap-allocations.html
//            


// Normal results.
// 
// ********************************************
// **  Rust benchmark String vs SmartString  **
// ********************************************
// Mary had a little lamb!
// Benchmark String len == 23: 1.000.000 => time: 81.011.780 nano sec.
//   res = 19000000
// Mary had a little lamb!
// Benchmark SmartString len == 23: 1.000.000 => time: 51.870.220 nano sec.
//   res = 19000000
// Mary had a little lamb!0123456789
// Benchmark String len > 23: 1.000.000 => time: 76.168.078 nano sec.
//   res = 19000000
// Mary had a little lamb!0123456789
// Benchmark SmartString len > 23: 1.000.000 => time: 79.027.405 nano sec.
//   res = 19000000


// Normal results with Link-Time Optimization 
//     with lto = true in Cargo.toml file.
// 
// ********************************************
// **  Rust benchmark String vs SmartString  **
// ********************************************
// Mary had a little lamb!
// Benchmark String len == 23: 1.000.000 => time: 62.688.124 nano sec.
//   res = 19000000
// Mary had a little lamb!
// Benchmark SmartString len == 23: 1.000.000 => time: 33.038.887 nano sec.
//   res = 19000000
// Mary had a little lamb!0123456789
// Benchmark String len > 23: 1.000.000 => time: 65.213.279 nano sec.
//   res = 19000000
// Mary had a little lamb!0123456789
// Benchmark SmartString len > 23: 1.000.000 => time: 65.253.016 nano sec.
//   res = 19000000


use std::time::{Instant, Duration};

fn main() {
    println!("********************************************");
    println!("**  Rust benchmark String vs SmartString  **");
    println!("********************************************");

    run_benchmarks();
}

//********************
//********************
// Benchmarks
//

fn run_benchmarks() {

    const MAX_ITERATIONS: usize = 1_000_000;
    // const MAX_ITERATIONS: usize = 3;

    // Bench string.
    let bench_1 = || -> usize {
        // Inner closure, the actual bench test
        let str_1 = "Mary had a little lamb!";
        let mut vec_tmp = Vec::with_capacity(MAX_ITERATIONS);
        let counter;
        let mut num = 0_usize;
        for i in 0..MAX_ITERATIONS {
            // let not_smart: String = str_1.to_string();
            let not_smart: String = String::from(str_1); 
            vec_tmp.push(not_smart);
            num += vec_tmp[i].find("lamb!").unwrap();
        }
        counter = vec_tmp.len() + num;
        println!("{}", vec_tmp[vec_tmp.len() - 1]);
        counter
    };    

    let (res_1, elapsed_1) = time_it(bench_1);
    println!("Benchmark String len == 23: {} => time: {} nano sec.",
            decimal_mark2(MAX_ITERATIONS.to_string()), 
            decimal_mark2(elapsed_1.subsec_nanos().to_string()));
    println!("  res = {}", res_1);

    smartstring::validate();

    // Board smart string.
    let bench_2 = || -> usize {
        // Inner closure, the actual bench test

        // NOTE: This substitutes in the scope of this function the String type.
        use smartstring::alias::String;
        let str_1 = "Mary had a little lamb!";
        let mut vec_tmp = Vec::with_capacity(MAX_ITERATIONS);
        let counter;
        let mut num = 0_usize;
        for i in 0..MAX_ITERATIONS {
            let smart_1 = String::from(str_1);
            vec_tmp.push(smart_1);
            num += vec_tmp[i].find("lamb!").unwrap();
        }
        counter = vec_tmp.len() + num;
        println!("{}", vec_tmp[vec_tmp.len() - 1]);
        counter
    };    

    let (res_2, elapsed_2) = time_it(bench_2);
    println!("Benchmark SmartString len == 23: {} => time: {} nano sec.",
                decimal_mark2(MAX_ITERATIONS.to_string()),
                decimal_mark2(elapsed_2.subsec_nanos().to_string()));
    println!("  res = {}", res_2);


    // Bench string greater then 23 characters.
    let bench_3 = || -> usize {
        // Inner closure, the actual bench test
        let str_1 = "Mary had a little lamb!0123456789";
        let mut vec_tmp = Vec::with_capacity(MAX_ITERATIONS);
        let counter;
        let mut num = 0_usize;
        for i in 0..MAX_ITERATIONS {
            // let not_smart: String = str_1.to_string();
            let not_smart: String = String::from(str_1); 
            vec_tmp.push(not_smart);
            num += vec_tmp[i].find("lamb!").unwrap();
        }
        counter = vec_tmp.len() + num;
        println!("{}", vec_tmp[vec_tmp.len() - 1]);
        counter
    };    

    let (res_3, elapsed_3) = time_it(bench_3);
    println!("Benchmark String len > 23: {} => time: {} nano sec.",
            decimal_mark2(MAX_ITERATIONS.to_string()), 
            decimal_mark2(elapsed_3.subsec_nanos().to_string()));
    println!("  res = {}", res_3);

    // Board smart string greater then 23 characters.
    let bench_4 = || -> usize {
        // Inner closure, the actual bench test

        // NOTE: This substitutes in the scope of this function the String type.
        use smartstring::alias::String;
        let str_1 = "Mary had a little lamb!0123456789";
        let mut vec_tmp = Vec::with_capacity(MAX_ITERATIONS);
        let counter;
        let mut num = 0_usize;
        for i in 0..MAX_ITERATIONS {
            let smart_1 = String::from(str_1);
            vec_tmp.push(smart_1);
            num += vec_tmp[i].find("lamb!").unwrap();
        }
        counter = vec_tmp.len() + num;
        println!("{}", vec_tmp[vec_tmp.len() - 1]);
        counter
    };    

    let (res_4, elapsed_4) = time_it(bench_4);
    println!("Benchmark SmartString len > 23: {} => time: {} nano sec.",
                decimal_mark2(MAX_ITERATIONS.to_string()),
                decimal_mark2(elapsed_4.subsec_nanos().to_string()));
    println!("  res = {}", res_4);

}

//********************
//********************
// Util functions
//

// Run function and return result with duration (seconds or nano seconds).
pub fn time_it<F, T>(f: F) -> (T, Duration)
        where F: FnOnce() -> T {
    
    let start = Instant::now();
    let res = f();
    let elapsed = start.elapsed();

    (res, elapsed)
}

fn decimal_mark2(s: String) -> String {
    let mut result = String::with_capacity(s.len() + ((s.len() - 1) / 3));
    let mut i = s.len();
    for c in s.chars() {
        result.push(c);
        i -= 1;
        if i > 0 && i % 3 == 0 {
            result.push('.');
        }
    }
    result
}
