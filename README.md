# evolutionary-algorithm

A rewrite from Python into Rust of the maximisation evolutionary algorithm implemented in this [tutorial](https://towardsdatascience.com/an-extensible-evolutionary-algorithm-example-in-python-7372c56a557b). The repository for that implementation is located [here](https://github.com/Garve/Evolutionary-Algorithm). 

By default, this program is set to maximise f(x)=-x(x-1)(x-2)(x-3)(x-4). This can be changed by modifying the fitness function in main().

I did this rewrite as I find it an effective way to learn new algorithms. As I had to translate the whole algorithm, it forced me to understand every line of its code.

I also added a feature to allow the user to add custom parameters at runtime.

This rewrite is also a lot quicker than the Python version - 108ms down to 2ms at default settings on my machine :)

## Compiling and Running

First, clone the repository and move into it.
```bash
$ git clone https://github.com/Sam-Bowden/evolutionary-algorithm
$ cd evolutionary-algorithm
```

Then compile and move into the directory where the compiled executable is located.
```bash
$ cargo build --release
$ cd /target/release
```

You can then run the executable "evo" with its default parameters. Try running it multiple times to see the solution returned varies per evolution.
```bash
$ ./evo
3.8170501705975353
$ ./evo
3.3332345727067088
```

You can also use the help argument to learn how to customise the parameters. With a good set of evolution parameters, you might be able to get a near perfect solution every time you run it!
```bash
$ ./evo --help
Usage: evo [OPTIONS]

Options:
      --pool-size <POOL_SIZE>                    [default: 10]
      --n-offsprings <N_OFFSPRINGS>              [default: 2]
      --n-epochs <N_EPOCHS>                      [default: 50]
      --pair-alpha <PAIR_ALPHA>                  [default: 0.5]
      --mutate-lower-bound <MUTATE_LOWER_BOUND>  [default: 0]
      --mutate-upper-bound <MUTATE_UPPER_BOUND>  [default: 4]
      --mutate-rate <MUTATE_RATE>                [default: 0.25]
      --init-lower-bound <INIT_LOWER_BOUND>      [default: 0]
      --init-upper-bound <INIT_UPPER_BOUND>      [default: 4]
  -h, --help                                     Print help information
  -V, --version                                  Print version information
```
