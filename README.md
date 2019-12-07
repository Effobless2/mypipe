# 🍌 mypipe ( | command rust overriding)

## Feature
 - Take a in command and an out command parameters. executes the first one and put its standard output in the standard input of the second one.

## Installation
```
cargo build
```

## Execution Examples
```
cargo run -- [ --in | -i ] "First Command" [ --out | -o ] "Second Command"
```

You can also add target/debug folder in your PATH variable and you can use the command "mypipe" on linux system and "mypipe.exe" on Windows system instead using cargo:
```
mypipe[.exe] --in "curl https://github.com/Effobless2" --out "grep Maxime"
```

For more infos :
```
mypipe[.exe] --help
```