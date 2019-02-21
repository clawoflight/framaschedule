# framaschedule
A library and command-line tool to schedule shifts based on poll responses - written in rust.

It currently only supports the CSV export from [Framadate](https://framadate.org/), hence the name.
However, it is modular and can easily be extended for other sources.

## Usage

```
USAGE:
    framaschedule [FLAGS] [OPTIONS] <POLLDATA>

FLAGS:
    -h, --help              Prints help information
    -f, --force-if-empty    Ignore slots that cannot be filled
    -V, --version           Prints version information

OPTIONS:
        --export-csv <output>    Output the best schedule in csv format

ARGS:
    <POLLDATA>    The csv file exported from framadate
```

By default, the best 2 schedules will be printed to `stdout`.
It is also possible to export the best schedule to a csv file, which can e.g. be opened in Excel.

If a shift can not be filled, the program will abort. However, a placeholder called `??` can be scheduled for unfillable shifts instead if required.

Even though this program finds the optimal solutions, which ones are printed is random because the order in which they are tried is not fixed (this is due to a non-deterministic seed in Rust's HashMap).


## Compiling and Running
Assuming you have the [rust toolchain](https://rustup.rs/) installed:

```bash
git clone https://github.com/clawoflight/framaschedule

cargo run --release -- $PATH_TO_CSV

# or:
cargo build --release
./target/release/framaschedule $PATH_TO_CSV
```



## Examples
For the following input:

``` csv
,"2019-02-24","2019-03-03","2019-03-10","2019-03-17",
,"12:00","18:00","12:00","18:00",
"Person1","Yes","Ifneedbe","No","Yes",
"Person2","No","Yes","Yes","No",
```

We get the following schedules:

```
24.02. 12:00:    Person1
03.03. 18:00:    Person2
10.03. 12:00:    Person2
17.03. 18:00:    Person1

Cost: 9.111111

Stats:
Person1: 2
Person2: 2

24.02. 12:00:    Person1
03.03. 18:00:    Person1?
10.03. 12:00:    Person2
17.03. 18:00:    Person1

Cost: 10.361111

Stats:
Person1: 3
Person2: 1

```


## Scheduling
Scheduling is implemented as (brute-force) global cost minimization, while drastically reducing the search space using the assumption that no-one will be scheduled much more often than the rest.
In order to somewhat speed up the search, it is parallelized over valid options for the first day using one thread for each physical cpu core.

The algorithm optimizes for three things, in decreasing priority:

1. Equal shift distribution between people (cost factor: ![square of occurences for each person](http://www.sciweavers.org/upload/Tex2Img_1550579799/eqn.png))
2. Equal distance between occurrences (cost factor: ![1 divided by square of average distance for each person](http://www.sciweavers.org/upload/Tex2Img_1550579726/eqn.png))
3. Minimal use of IfNeedBe responses (cost factor: 0.25 for every use)

This approach is not suited for large problem sizes or custom constraints.
If that is your use-case, consider a heuristics-based system such as [OptaPlanner](https://www.optaplanner.org/).
