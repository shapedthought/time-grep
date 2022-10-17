# time-grep

A bit like ripgrep but allows you to set a last modified time. It recursively looks through all files in a directory.

It uses regular expressions (Regex) for the pattern search terms.

    USAGE:
        time-grep [OPTIONS] --path <PATH> --search-term <SEARCH_TERM> --mins <MINS>

    OPTIONS:
        -h, --help                         Print help information
        -m, --mins <MINS>                  modified time in the past in min
        -p, --path <PATH>                  path
        -s, --search-term <SEARCH_TERM>    search term regex
        -f --file_only                     only prints the filename with a match

## Examples
    
Find the expression "17.10.2022 this is an Error", searching for the timestamp and "Error" string, modified in the last 30 min

    time-grep.exe --path /path/to/search --search-term "^.*?\b17.10.2022\b.*?\bError\b.*?$" --mins 30 

There is a log file with the structure "17.10.2022 08:32:11 this is an Error". We want to find all entries for 17.10.2022 at 8am, with the file modified in the last 30 min

    time-grep.exe -p /path/to/search -s "^.*?\b17.10.2022\b.*?\s08:.*?\bError\b" -m 30

Find "Error" anywhere in the string, modified in the last 30 min

    time-grep.exe -p /path/to/search -s "Error" -m 30

Find any combination of "Error" using the regex case insensitivy flag, modified in the last 10 min

    time-grep.exe -p /path/to/search -s "(?i)Error" -m 10

Search for an email string, modified in the last hour

    time-grep.exe -p /path/to/search -s "(\w+)@(\w+).(\w+)" -m 60

Search for "Error" or "Warning", modified in the last 20 min

    time-grep.exe -p /path/to/search -s "\bWarning\b|\bError\b" -m 20

## Install

- Download this git
- run "cargo build --release"
- executable will be in the /target/src folder
