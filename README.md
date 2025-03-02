![Icarus](./doc/img/icarus-banner.webp)

# Icarus

A Command and Control server and his agent written in Rust🦀!

> [!note]
> This project is a complete rewrite of [this project](https://github.com/Code-Barru/old_icarus).

> [!warning]
> 🚧This project is still in development!

## Features

- 👨‍💻 AES-256 encrypted traffic
- 🔃 Agent auto update
- 🤠 Agent persitence
- 📔 Task Queuing

## Features to come

- 🚀 AV & EDR evasion
- 🐱‍👤 Agent Stealth Infection
- ☢ Agent nuke
- 📁 File Transfer
- 📸 Screenshot
- 💻 Web Interface
- 🤖 Auto Deployment

## Installation

### Pre-requisites

Have Rust installed on your machine. You can install it by following the instructions on the [official website](https://www.rust-lang.org/tools/install).

### Clone the repository

```sh
git clone https://github.com/Code-Barru/icarus.git
cd icarus
```

Change the Server addr in `setup/main.rs`.

### Build the project

```sh
cargo build --release
```

### Launch the database

```sh
docker-compose up -d
```

### Setup environment variables

```sh
export DATABASE_URL=postgres://icarus:icarus@localhost/icarus
export RSA_PRIVATE_KEY_PATH=path/to/private_key.pem # Optional, default is private_key.pem
export RUST_LOG={info,debug,error,trace} # Optional, default is info
```

Can also be set in a `.env` file.

### Apply the migrations

```sh
cd server
diesel migration run
```

### Create server distribution folder && copy binaries

```sh
mkdir dist
cp ../target/release/agent.exe dist/
cp ../target/release/setup.exe dist/
```

### Launch the server

```sh
cargo run -p server --release
```

You now just have to upload "setup" to the target !

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Legal disclaimer

Usage of this tool for attacking targets without prior mutual consent is illegal. It's the end user's responsibility to obey all applicable local, state and federal laws. Developers assume no liability and are not responsible for any misuse or damage caused by this program.
