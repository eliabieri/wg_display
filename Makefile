all: app

run: app_debug

clean:
	rm -rf target
	rm -rf src/configuration_frontend/dist*

## Build the application
dependencies = \
	src/main.rs \
	src/renderer/mod.rs

# Build complete app for the native platform
target/release/wg_display: $(dependencies) configuration_frontend
	cargo build --release
app: target/release/wg_display

# Run complete app
target/debug/wg_display: $(dependencies) configuration_frontend
	cargo run
app_debug: target/debug/wg_display

# Build complete app for arm (Raspberry Pi 2/3/4)
target/armv7-unknown-linux-gnueabihf/wg_display: $(dependencies) configuration_frontend
	cross build --release --target armv7-unknown-linux-gnueabihf
app_armv7: target/armv7-unknown-linux-gnueabihf/wg_display

# Build complete app for arm (Raspberry Pi 0/1)
target/arm-unknown-linux-gnueabihf/wg_display: $(dependencies) configuration_frontend
	cross build --release --target arm-unknown-linux-gnueabihf
app_arm: target/arm-unknown-linux-gnueabihf/wg_display

## Build frontend using trunk
dependencies = \
	src/configuration_frontend/src/main.rs \
	src/configuration_frontend/index.html

src/configuration_frontend/dist/css/output.css: $(dependencies)
	cd src/configuration_frontend && npm run tailwind-build
tailwindcss: src/configuration_frontend/dist/css/output.css

configuration_frontend: tailwindcss
	cd src/configuration_frontend && trunk build --release --dist dist/yew