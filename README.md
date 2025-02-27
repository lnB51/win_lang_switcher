#### Windows lang switcher like on MacOS/Linux
![Version 0.1](https://img.shields.io/badge/Version%200.1-FFC832?style=for-the-badge&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-000?style=for-the-badge&logo=rust&logoColor=white)
[![MIT License](https://img.shields.io/badge/MIT%20License-004772?style=for-the-badge&logo=license&logoColor=white)](https://github.com/lnB51/spark/blob/master/LICENSE)
<p>
  This project aims to solve the problem of comfortably changing the language on Windows. Although Windows provides a language change preview, it may not be comfortable for all users, this project provides an interface very similar to the interface of macOS and Linux.
</p>

## Prerequisites

### Install Rust (with `rustup`)

If you don't have `rustup` installed yet, follow the instructions on the [rustup.rs site](https://rustup.rs)

### Install Tauri CLI (with `cargo`)

```sh
cargo install tauri-cli --version "^2.0.0" --locked
```

## Let's get started

#### Copy source code
```sh
git clone https://github.com/lnB51/win_lang_switcher
```

#### Install npm packages

Install `npm` packages reqired for this project (in root dir)

```sh
npm i
```

#### Build (Debug)

Run debug with Tauri CLI

```sh
cargo tauri dev
```

#### Build (Release)

To skip bulding installers run (create pure executable)

```sh
cargo tauri build -b --no-bundle
```

If you need installer for your app run (create two different types of installers msi and exe)

```sh
cargo tauri build
```

#### Run

To run app without installers run in cmd (root dir)

```sh
.\target\release\app.exe
```

PS: You can add this executable to your startup to run automatically when your system boots.
