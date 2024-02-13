# Silly Strings
Silly Strings is a CLI program based off the Unix `strings` command. It allows for the discovery of string contents within any file (text files, binary files, etc.).
The program doesn't implement every feature in the original `strings` command, but it can do other things such as filtering for an exact or 
regular expression pattern or listing only unique strings. Currently, Silly Strings offers support for ASCII and UTF-8 encodings.
# Usage
```
Analyze and display strings in a file

Usage: silly-strings [OPTIONS] <PATH>

Arguments:
  <PATH>  The path to the file

Options:
  -e, --encoding <ENCODING>  Parse the file with the given encoding [supports: ASCII, UTF-8] [default: ASCII]
  -l, --lowercase            Convert all strings to lowercase
  -m, --match <SUBSTRING>    Define an exact match substring to filter strings [default: ]
  -n, --bytes <MIN_LEN>      Minimum number of bytes considered a string [default: 4]
  -r, --regex <REGEX>        Define a regex pattern to filter strings [default: ]
  -t, --radix <RADIX>        Print the location of each string in (o)ctal, (d)ecimal, or he(x) [default: n]
  -u, --uniq                 Only print unique strings (matching strings are merged to the first instance)
  -h, --help                 Print help
  -V, --version              Print version
```
