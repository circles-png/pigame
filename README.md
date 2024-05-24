# pigame

not be confused with `pygame`

pi zero w + rust + hdmi output + some buttons => game console

macroquad-like api (global state)

# installation

submodule this repo into your project

```bash
git submodule add https://github.com/circles-png/pigame
```

# usage

set up env

## setup part I - deploy script

make a `deploy.sh` file in the root of your project with the following contents

```sh
set -e
set -o errexit
set -o nounset
set -o pipefail
set -o xtrace
cargo fmt
cargo build --bin <binary name>
scp target/arm-unknown-linux-musleabihf/debug/<binary name> pi@raspberrypi.local:~
ssh -t pi@raspberrypi.local "RUST_BACKTRACE=1 ~/<binary name>"
```

where `<binary name>` is the name of the binary you want to deploy

## setup part II - build config

make a `.cargo/config.toml` file in the root of your project with the following contents

```toml
[build]
target = "arm-unknown-linux-musleabihf"

[target.arm-unknown-linux-musleabihf]
linker = "arm-unknown-linux-musleabihf-gcc"
```

or use another linker and target if you know what you're doing

if you're using vscode, you can make a `.vscode/tasks.json` to run the deploy script

```json
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "deploy",
            "type": "shell",
            "command": "./deploy.sh",
            "group": {
                "kind": "build",
                "isDefault": true
            }
        }
    ]
}
```
