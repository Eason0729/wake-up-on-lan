# wake-up-on-lan

A simple web site to wake up your computer when you are not home.

![demo image](https://user-images.githubusercontent.com/30045503/184494866-f941a2be-e275-4ec9-bf91-ddf95cea7c8a.png)

## Get Started

1. Download achieve files from [release](https://github.com/Eason0729/wake-up-on-lan/releases), and uncompress(unzip) it.
2. Edit ``.env`` file(or set environment variable)

```env
PORT=80 # required
MAC_ADDRESS=b28116f31e6c # required
PASSWORD=abcdefg # required if HASHED_PASSWORD is not presented
ADDRESS='0.0.0.0' # optional
HASHED_PASSWORD='$argon2i$v=19$m=4096,t=3,p=1$hfcSURRz3uackLG9Kb1Z/g$NQM4G7sWd2xT9laJCdmkwDoSV0/i5KL6aBKnETHC4Cg' # optional
```

3. Run the program

```shell
$ ./wake-up-on-lan
```

## Advance Features

### Use hashed password

1. Hash password

```shell
$  echo -n "abcdefg" | npx argon2-cli -e
$argon2i$v=19$m=4096,t=3,p=1$hfcSURRz3uackLG9Kb1Z/g$NQM4G7sWd2xT9laJCdmkwDoSV0/i5KL6aBKnETHC4Cg
```

2. Edit ``.env`` file

Remember to put it inside single quotes(``'``).

```env
PORT=80
MAC_ADDRESS=b28116f31e6c
HASHED_PASSWORD='$argon2i$v=19$m=4096,t=3,p=1$hfcSURRz3uackLG9Kb1Z/g$NQM4G7sWd2xT9laJCdmkwDoSV0/i5KL6aBKnETHC4Cg'
```

### Bind address

Set "ADDRESS" environment variable

### Docker

```shell
$  docker build . --build-arg TARGET=x86_64-unknown-linux-musl -t wake-up-on-lan
$  docker run --net=host -d -e PORT=8080 -e MAC_ADDRESS=b28116f31e6c -e PASSWORD=abcdefg wake-up-on-lan
```
