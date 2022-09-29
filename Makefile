configuration_frontend_dist = src/configuration_frontend/dist
tailwind_output_css = $(configuration_frontend_dist)/output.css
yew_index_html = $(configuration_frontend_dist)/index.html
yew_build = $(yew_index_html)

build_native_release = target/release/wg_display
build_native_release_debug = target/debug/wg_display

$(build_native_release_debug):

.PHONY: clean
clean:
	rm -rf target
	rm -rf src/configuration_frontend/dist*

## Build the application
dependencies = \
	src/main.rs \
	src/renderer/mod.rs

# Build complete app for the native platform
$(build_native_release): $(dependencies) $(yew_build)
	cargo build --release
app: $(build_native_release)

# Build complete app for the native platform in debug mode
$(build_native_release_debug): $(dependencies) $(yew_build)
	cargo build
app_debug: $(build_native_release_debug)

# Build complete app for arm (Raspberry Pi 2/3/4)
target/armv7-unknown-linux-gnueabihf/wg_display: $(dependencies) $(yew_build)
	cross build --release --target armv7-unknown-linux-gnueabihf
app_armv7: target/armv7-unknown-linux-gnueabihf/wg_display

# Build complete app for arm (Raspberry Pi 0/1)
target/arm-unknown-linux-gnueabihf/wg_display: $(dependencies) $(yew_build)
	cross build --release --target arm-unknown-linux-gnueabihf
app_arm: target/arm-unknown-linux-gnueabihf/wg_display

## Build frontend using trunk
dependencies = \
	src/configuration_frontend/src/main.rs \
	src/configuration_frontend/index.html \
	src/configuration_frontend/package.json
$(tailwind_output_css): $(dependencies)
	# Force regeneration
	rm -rf $(tailwind_output_css)
	cd src/configuration_frontend && npm run tailwind-build

$(yew_build): $(tailwind_output_css) $(dependencies)
	cd src/configuration_frontend && trunk build --release