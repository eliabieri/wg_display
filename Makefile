# Make does not offer a recursive wildcard function, so here's one:
rwildcard=$(wildcard $1$2) $(foreach d,$(wildcard $1*),$(call rwildcard,$d/,$2))

frontend_dist = frontend/dist
tailwind_output_css = $(frontend_dist)/$(wildcard output-*.css)
yew_index_html = $(frontend_dist)/index.html
frontend_build = $(yew_index_html)

build_native_release = app/target/release/app
build_native_release_debug = app/target/debug/app

app:

.PHONY: clean
clean:
	rm -rf app/target
	rm -rf common/target
	rm -rf frontend/dist

## Build the application
dependencies = \
	$(call rwildcard,app/src/,*.rs) \
	$(call rwildcard,common/src/,*.rs) \
	$(call rwildcard,frontend/src,*.rs) \
	$(call rwildcard,app/src/,*.rs) \
	$(call rwildcard,common/src/,*.rs) \
	$(call rwildcard,app/,Cargo.*) \
	$(call rwildcard,common/,Cargo.*) \
	$(call rwildcard,frontend/,Cargo.*)

# Generate docs
docs: $(dependencies) $(frontend_build)
	cd app && cargo doc --no-deps
	cd common && cargo doc --no-deps
	cd frontend && cargo doc --no-deps

# Build complete app for the native platform
$(build_native_release): $(dependencies) $(frontend_build)
	cd app && cargo build --release
app: $(build_native_release)

# Build complete app for the native platform in debug mode
$(build_native_release_debug): $(dependencies) $(frontend_build)
	cd app && cargo build
app_debug: $(build_native_release_debug)

# Build complete app for arm (Raspberry 4, 64 bit)
target/aarch64-unknown-linux-gnu/app: $(dependencies) $(frontend_build)
	cd app && cross build --release --target aarch64-unknown-linux-gnu
app_aarch64: target/aarch64-unknown-linux-gnu/app

# Build complete app for arm (Raspberry Pi 2/3/4)
target/armv7-unknown-linux-gnueabihf/app: $(dependencies) $(frontend_build)
	cd app && cross build --release --target armv7-unknown-linux-gnueabihf
app_armv7: target/armv7-unknown-linux-gnueabihf/app

# Build complete app for arm (Raspberry Pi 0/1)
target/arm-unknown-linux-gnueabihf/app: $(dependencies) $(frontend_build)
	cd app && cross build --release --target arm-unknown-linux-gnueabihf
app_arm: target/arm-unknown-linux-gnueabihf/app

## Build frontend using trunk
dependencies = \
	$(call rwildcard,frontend/src/,*.rs) \
	frontend/index.html \
	frontend/package.json
$(tailwind_output_css): $(dependencies)
	# Force regeneration
	rm -rf $(tailwind_output_css)
	cd frontend && npm run tailwind-build

$(frontend_build): $(tailwind_output_css) $(dependencies)
	cd frontend && trunk build --release