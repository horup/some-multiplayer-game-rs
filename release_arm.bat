wasm-pack build --target web --out-dir public/pkg
cross build --release --target arm-unknown-linux-gnueabihf
copy target\arm-unknown-linux-gnueabihf\release\some-multiplayer-game some-multiplayer-game