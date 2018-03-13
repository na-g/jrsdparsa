.PHONY: build

TARGETDIR=target/wasm32-unknown-unknown/release
build:
	cargo +nightly build --verbose --all --release --target wasm32-unknown-unknown
	wasm-bindgen ${TARGETDIR}/jrsdparsa.wasm --out-dir ${TARGETDIR}
	# Hack for chrome support
	wasm2es6js ${TARGETDIR}/jrsdparsa_bg.wasm -o ${TARGETDIR}/jrsdparsa_bg.js --base64
	rm ${TARGETDIR}/jrsdparsa_bg.wasm