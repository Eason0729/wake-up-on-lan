# wake-up-on-lan

A simple web site to wake up your computer when you are not home.

![image](https://user-images.githubusercontent.com/30045503/179151785-dd48b5a7-72c3-4163-8485-623e74350690.png)

## How to use

1. Clone the repo
2. Download achieve files from release, and uncompress it.
3. Then, the folder structure should look like this.

```shell
~ tree .
.
├── config.json
├── public
│   └── index.html
├── wake-up-on-lan
└── wake-up-on-lan-aarch64-unknown-linux-gnu.tar.gz
```

4. Edit ``config.json``

```javascript
{
    "password":"my awsome password",// password
    "mac":[1,1,1,1,1,1],// mac address of the computer you want to wake up
    "port":80// port
}
```

5. Run wake-up-on-lan

```shell
./wake-up-on-lan
```

## Goals

I made this project to learn:

1. CI/CD
2. raw UDP in Rust
