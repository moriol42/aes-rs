# aes-rs

A work in progress AES-128 implementation written in rust.

**Warning: Do NOT use this code in production, it may contain security flaws[^1].**

## Usage:

```shell
Usage: aes-rs [OPTIONS] --file <FILE> --key <KEY>

Options:
  -e, --encrypt      
  -d, --decrypt      
  -f, --file <FILE>  Input file
  -o, --out <OUT>    Output file
  -k, --key <KEY>    Key file
      --ecb          ECB mode
      --cbc          CBC mode
  -h, --help         Print help
  -V, --version      Print version
```


[^1]: and is likely to be vulnerable to [timing attacks](https://en.wikipedia.org/wiki/Timing_attack)
