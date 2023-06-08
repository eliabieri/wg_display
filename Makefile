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
	cd app && cargo clean
	cd common && cargo clean
	cd frontend && cargo clean

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

# Build complete app for ARMv8 (Raspberry Pi 3, 4 and Zero 2 W with 64-bit OS)
target/aarch64-unknown-linux-gnu/app: $(dependencies) $(frontend_build)
	cd app && cross build --release --target aarch64-unknown-linux-gnu
app_aarch64: target/aarch64-unknown-linux-gnu/app

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