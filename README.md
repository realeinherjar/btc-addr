# btc-addr - Generate Bitcoin addresses from a Bitcoin XPUB

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A no-brainer CLI to generate Bitcoin addresses from a BIP84 XPUB.

## Usage

```bash
$ btc-addr --help
A simple command line tool to generate Bitcoin addresses from a XPUB

Usage: btc-addr [OPTIONS] --xpub <XPUB> --fingerprint <FINGERPRINT>

Options:
  -x, --xpub <XPUB>                Extended public key
  -f, --fingerprint <FINGERPRINT>  Fingerprint of the master key
  -s, --single <SINGLE>            Index of the address to generate
  -r, --range <RANGE>              Range of addresses to generate (start-end)
  -h, --help                       Print help
  -V, --version                    Print version
```

You can either pass a single address index to generate or a range of indexes.
Examples:

- Single address:

  ```bash
  $ btc-addr -x xpub6Cbhm1egsKZ8PAPcW6YAy2s5eEm4WsW2oEmcVeNe3A51yn69zff3A8EfDr2pFCd4iYr6Lsi5EhdRHZskkSEo28LLDq1tqCyVG8NqWMizgcB -f 636f9859 -s 0
  bc1qklsp8d45494wgkyrr64mnhdsrmlcp3rmq99ayg
  ```

- Range of addresses:

  ```bash
  $ btc-addr -x xpub6Cbhm1egsKZ8PAPcW6YAy2s5eEm4WsW2oEmcVeNe3A51yn69zff3A8EfDr2pFCd4iYr6Lsi5EhdRHZskkSEo28LLDq1tqCyVG8NqWMizgcB -f 636f9859 -r 0-9
  bc1qklsp8d45494wgkyrr64mnhdsrmlcp3rmq99ayg
  bc1q89ahz76lnerxzqjmftrwf34gjv9d8s02n7r47g
  bc1qzfxcrwg7mn3lv953w3gz9cyqtv5j36valwpzl3
  bc1qmrs7uxsz5t9lulvfnzzg34hf2fjyt8ql0pyr6k
  bc1qxp59fk9tksp5huw5k65zdvs94mn5t3uk8dqqtt
  bc1qsgqq5alccmq5p6udqkm72yu3p0rlrl00jhemyd
  bc1qul6nek06m3mj427eatm3ln5kk0shgex2tg9m8d
  bc1qdlgjqn9qsqqct6ytc2xr54wn57jydztgfkvzjl
  bc1qwgt2p3p6c3v56nxs2s7z9xdm4aexu3haq0zv8u
  bc1qqh745peskqrqvr24fq0km3jp4fatf0dvr38udw
  ```
