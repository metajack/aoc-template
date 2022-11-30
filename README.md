This is a simple template for AOC solutions.

To start a new day (except for day1 which is stubbed already):

1. cp src/DAY.rs src/dayX.rs
2. Add `mod dayX;` near the top of main.rs.
3. In main.rs add a new branch to `main()` like `X => run_day!(dayX, dayX::DayX, &args.input),` near the others.
4. For the `DayX` struct that day, use whatever fields your inputs requires; this will often been some Vec field.
5. Implement `from_input()` to parse the input string into your `DayX` struct. Iterator functions are great for this.
6. Implement `solve_part1()`.
7. Implement `solve_part2()`.

Run your solution with `cargo run -- --day X --part P input/dayX.input`

You can also use [aoc-cli](https://crates.io/crates/aoc-cli) to manage your inputs. If you do, download the day's input with `aoc -y Y -d D download`.

Good luck!

