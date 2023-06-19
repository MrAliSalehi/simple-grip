# simple-grip
simple `grip-like` program that allows you to search files/directories with a specific pattern

# install

1. download the latest stable version from [releases](https://github.com/MrAliSalehi/simple-grip/releases/latest)
2. build from source code:
- `git clone https://github.com/MrAliSalehi/simple-grip`
- `cargo build --release`

# use
```
  <PATH>     base path to search
  <PATTERN>  regex pattern, its recommended to wrap it around single quotes(')

Options:
  -r, --reverse        show anything that doesnt match the patterns
  -s <TARGET>          search target, can either be name of the files/directories or file contents [default: names] [possible values: names, contents]
  -t <MAX_THREAD>      set max thread count [default: 0]
  -d <MAX_DEPTH>       max directory depth to search [default: 3]
  -h, --help           Print help
  -V, --version        Print version
```
- for example:

`simple-grip /path/to/dir 'pattern\+' -s names -t 3 -d 4`

with this command simple grip will go in `/path/to/dir` and search for the files/directories that matches the `'pattern\+'` with maximum 3 threads and `tree-depth` of 3.

you can also `reverse` the process so any file that doesn't match the `'pattern\+'` will show up, in order to do this you can simply add the `-r` to the end of the command
