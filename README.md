# Rust benchmark String vs SmartString and LTO
Some really nice optimizations :-)

## Description 
This little project tests 2 things. The first one is a benchmark between the the creation of 1.000.000 (million) strings with a normal Rust String type and a SmartString (also SmallString and SmallStr). I tested for a string length lower or equal to 23 chars and for a string with longer length. This size is important because SmartString has no Heap allocation for small strings, if it's length is lower or equal to 23 bytes (ex: 23 ASCII chars). The difference in performance is huge in that case! And contrary what I have read in the SmartString documentation for bigger string sizes the performance is the same. At least if you do what I did, turn on the LTO - Link-Time Optimization flag for longer strings, the processing times are also equal. <br>
<br>
The second thing that this benchmark tests is the performance uplift with and with out LTO - Link-Time Optimization flag compilation flag. The compilation times for release build are a little longer, but the performance uplift is huge! This without changing one line of code! So is more performance for free :-)


## Usage note 

```
When creating SmartString's from &str instead of doing:

    "blabla".to_string()

Do:

    String::from("blabla")
```


## Benchmark Results

**Normal results** <br>

```
********************************************
**  Rust benchmark String vs SmartString  **
********************************************
Mary had a little lamb!
Benchmark String len == 23: 1.000.000 => time: 82.497.024 nano sec.
Mary had a little lamb!
Benchmark SmartString len == 23: 1.000.000 => time: 52.576.317 nano sec.
Mary had a little lamb!
Benchmark SmallString len == 23: 1.000.000 => time: 117.569.002 nano sec.
Mary had a little lamb!
Benchmark SmallStr len == 23: 1.000.000 => time: 56.101.449 nano sec.

Mary had a little lamb!0123456789
Benchmark String len > 23: 1.000.000 => time: 89.052.373 nano sec.
Mary had a little lamb!0123456789
Benchmark SmartString len > 23: 1.000.000 => time: 72.478.181 nano sec.
Mary had a little lamb!0123456789
Benchmark SmallString len > 23: 1.000.000 => time: 134.537.971 nano sec.
Mary had a little lamb!0123456789
Benchmark SmallStr len > 23 in stack: 1.000.000 => time: 31.366.496 nano sec.
Mary had a little lamb!0123456789
Benchmark SmallStr len > 23 in heap: 1.000.000 => time: 70.096.575 nano sec.
```
<br>

**Normal results with Link-Time Optimization** <br> 
with ```lto = true``` in ```Cargo.toml``` file. <br>

```
********************************************
**  Rust benchmark String vs SmartString  **
********************************************
Mary had a little lamb!
Benchmark String len == 23: 1.000.000 => time: 62.591.134 nano sec.
Mary had a little lamb!
Benchmark SmartString len == 23: 1.000.000 => time: 33.567.699 nano sec.
Mary had a little lamb!
Benchmark SmallString len == 23: 1.000.000 => time: 118.631.379 nano sec.
Mary had a little lamb!
Benchmark SmallStr len == 23: 1.000.000 => time: 38.044.226 nano sec.

Mary had a little lamb!0123456789
Benchmark String len > 23: 1.000.000 => time: 69.761.895 nano sec.
Mary had a little lamb!0123456789
Benchmark SmartString len > 23: 1.000.000 => time: 54.457.676 nano sec.
Mary had a little lamb!0123456789
Benchmark SmallString len > 23: 1.000.000 => time: 149.197.473 nano sec.
Mary had a little lamb!0123456789
Benchmark SmallStr len > 23 in stack: 1.000.000 => time: 43.619.707 nano sec.
Mary had a little lamb!0123456789
Benchmark SmallStr len > 23 in heap: 1.000.000 => time: 62.965.861 nano sec.
```

**SmartString** and **SmartStr** are the **clear winners** in this test case for small strings and don't have any penalty for bigger strings.


## References
* **Crate SmartString** <br>
  [https://crates.io/crates/smartstring](https://crates.io/crates/smartstring)

* **How to learn modern Rust - Compilation flags** <br>
[https://github.com/joaocarvalhoopen/How_to_learn_modern_Rust#rust-optimization---compilation-modes-and-flags](https://github.com/joaocarvalhoopen/How_to_learn_modern_Rust#rust-optimization---compilation-modes-and-flags)

* **The Rust Performance Book - Heap Allocations** <br>
  [https://nnethercote.github.io/perf-book/heap-allocations.html](https://nnethercote.github.io/perf-book/heap-allocations.html)


## License
MIT Open Source License


## Have fun!
Best regards, <br>
Joao Nuno Carvalho

