frontend_dist = frontend/dist
tailwind_output_css = $(frontend_dist)/$(wildcard output-*.css)
yew_index_html = $(frontend_dist)/index.html
frontend_build = $(yew_index_html)

build_native_release = app/target/release/wg_display
build_native_release_debug = app/target/debug/wg_display

app:

.PHONY: clean
clean:
	rm -rf app/target
	rm -rf src/frontend/dist*

## Build the application
dependencies = \
	app/src/main.rs \
	app/src/renderer/mod.rs

# Build complete app for the native platform
$(build_native_release): $(dependencies) $(frontend_build)
	cd app && cargo build --release
app: $(build_native_release)

# Build complete app for the native platform in debug mode
$(build_native_release_debug): $(dependencies) $(frontend_build)
	cd app && cargo build
app_debug: $(build_native_release_debug)

# Build complete app for arm (Raspberry Pi 2/3/4)
target/armv7-unknown-linux-gnueabihf/wg_display: $(dependencies) $(frontend_build)
	cd app && cross build --release --target armv7-unknown-linux-gnueabihf
app_armv7: target/armv7-unknown-linux-gnueabihf/wg_display

## Build frontend using trunk
dependencies = \
	frontend/src/main.rs \
	frontend/index.html \
	frontend/package.json
$(tailwind_output_css): $(dependencies)
	# Force regeneration
	rm -rf $(tailwind_output_css)
	cd frontend && npm run tailwind-build

$(frontend_build): $(tailwind_output_css) $(dependencies)
	cd frontend && trunk build --release