# stack genetic programming test

This is an exercise to use Stack GP for a simple symbolic regression task.

## Crates used

 - `rand` is used as an RNG
 - `rayon` is used to use multiple threads and thus higher performance
 - `clap` is used to parse command line arguments. You can specify the population and other hyper parameters

## Known issues

 - It seems that on some computers, `rayon` detect 2 processors rather than 4. There are actually 2 processors but the CPU supports 4 threads. This technology is known as hyper threading. To overcome this, you might want to specify number of processors used manually in the code.

## What SR problem does it solve?

`f(x) = 2 * x * x`

## Since this software is written in Rust, is it idiomatic?

No, not yet. I have just got the thing working and haven't got time to ask Rust community to do a code review for me, yet.


## License

The MIT License. Copyright (C) 2023 Farooq Karimi Zadeh

`fkz@riseup.net`
