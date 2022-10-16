# time-grep

A bit like ripgrep but allows you to set a last modified time. 

    USAGE:
        time-grep [OPTIONS] --path <PATH> --search-term <SEARCH_TERM> --mins <MINS>

    OPTIONS:
        -h, --help                         Print help information
        -i, --insensitive                  case insensitive flag
        -m, --mins <MINS>                  modified time in the past in min
        -p, --path <PATH>                  path
        -s, --search-term <SEARCH_TERM>    search term

## Examples

    time-grep.exe --path /path/to/search --search-term Error -mins 30 --insensitive

    time-grep.exe -p /path/to/search -s Error -m 30 -i

## Install

- Download this git
- run "cargo build --release"
- executable will be in the /target/src folder