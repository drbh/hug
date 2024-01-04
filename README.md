# hug

HuggingChat desktop app.

https://github.com/drbh/hug/assets/9896130/f702154a-fd17-4d38-a95e-f07b0e9af393

### Development/Build

```bash
# install tauri
cargo install --git https://github.com/tauri-apps/tauri --branch 1.x tauri-cli
# build app; copy the app to the /Applications folder
make build
# run app (cli, spotlight, etc.)
open -a /Applications/hug.app
```
