# rustContainer
a simple Rust dev container/environment to play around without installing anything (mainly for windows)

## use


### build
You have to build the container first.

Windows
```
.\buildContainer.bat
```

Linux
```
./buildContainer.sh
```

### run

to run the container and get into the interactive sgell execute

Windows
```
.\start.bat
```

Linux
```
./start.sh
```

## cross-compile

### windows

To create a windows build from within the container, launche the container, cd into the project directory and run

```bash
compile-win
```

The windos executable can be found in
```
target/x86_64-pc-windows-gnu/release/my_app.exe
```
