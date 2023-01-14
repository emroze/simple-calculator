# simple-calculator
Simple rust calculator

# Usage:

First, change the directory to the simple-calculator directory.

To build the file, run 
```sh
cargo build --release
```

Use: 
```sh
target/release/simple-calculator first_value operator second_value
```

There are 4 operators. They are:
```
 "Operation"      =  "Character"
 Addition         =   '+'
 Subtract         =   '-'
 multiplication   =   '×', 'X'
 Division         =   '/'
```

For multiplying 12 by 5.5 you can use 
```sh
target/release/simple-calculator 12 x 5
```