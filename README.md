# irrec-rs

IR Receiver and Decoder for Raspberry Pi.

## prerequirement

### install tool

```sh
$ cargo install cargo-make
```

### create `.env` file

```sh
$ cp sample.env .env
```

## run

dev

```sh
$ cargo make run
```

release

```sh
$ cargo make run --profile release
```


NEC: 562 x 16 = 8992 (us)
AEHA: (350, 425, 500) * 8 = 2800, 3400, 4000
SONY: 600 * 4 = 2400