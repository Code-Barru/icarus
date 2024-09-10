![Icarus](./img/Icarus-background.png)

# Icarus

A Command and Control server and his agent written in Rust🦀!

Web interface written using SvelteKit and Tailwindcss!

> [!warning]
> 🚧This project is still in development!🚧

## Features

### TODO

- [ ] Upload payload
- [ ] ReverShell in web app 👾
- [ ] Nuke the client ☢

### In Progress

- [ ] Web Interface 💻

### Done

- [x] Task queuing 📃

## Prerequisite

Having `git`, `cargo` and `docker` installed.

Change `REMOTE ADDRESS` address in `agent/main.rs`

## How to use

```shell
git clone https://github.com/Code-Barru/icarus.git
cd icarus
docker-compose up -d    # launches the server
cd agent && cargo build # builds the agent
# upload the agent to target machine
# ???
# profit
```

## Legal Disclaimer

The information provided in this project is for educational and informational purposes only. The authors and contributors of this project are not responsible for any misuse or illegal activities conducted using this software. The use of this software is at your own risk. Please ensure compliance with all applicable laws and regulations in your jurisdiction. By using this software, you agree to indemnify and hold harmless the authors and contributors from any claims, damages, or liabilities arising out of your use of the software.
