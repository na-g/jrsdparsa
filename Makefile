.PHONY: build

TARGETDIR=target/wasm32-unknown-unknown/release
build:
	cargo +nightly build --verbose --all --release --target wasm32-unknown-unknown
	wasm-bindgen ${TARGETDIR}/jrsdparsa.wasm --out-dir ${TARGETDIR}
	wasm2es6js ${TARGETDIR}/jrsdparsa_wasm.wasm -o ${TARGETDIR}/jrsdparsa_wasm.js --base64
