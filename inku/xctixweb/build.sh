#!/bin/bash
#

hash_files(){
    need_cmd tree
    need_cmd find
    need_cmd sha256sum
    tree -h > docs/src/filetree.txt
    find $PWD -type f ! -regex '\./[^.]*$' -exec sha256sum {} \; > docs/src/SHA256SUMS
}

need_cmd() {
    if ! check_cmd "$1"; then
        err "need '$1' (command not found)"
    fi
}

check_cmd() {
    command -v "$1" > /dev/null 2>&1
}

err() {
    say "$1" >&2
    exit 1
}

say() {
    printf "TRIER: %s\n" "$1"
}

cp_dir(){
    local FROM=$1
    local TO=$2
    rm -rf $TO
    mkdir -p $TO
    cp -R $FROM/. $TO
}

BUILD_LOG=target/build-sh.log

build_wasm_webfoo(){
    echo "=== Build wasm webfoo" >> $BUILD_LOG
    # [Add support for emitting a Wasm Interface Types section by alexcrichton · Pull Request #1725 · rustwasm/wasm-bindgen](https://github.com/rustwasm/wasm-bindgen/pull/1725)
    # [No module named '__wbindgen_placeholder__' · Issue #2 · bytecodealliance/wasmtime-demos](https://github.com/bytecodealliance/wasmtime-demos/issues/2)
    pushd wasm/webfoo
    rm -rf pkg
    # Build zero-js-glue version webfoo_lib.wasm
    WASM_INTERFACE_TYPES=1 wasm-pack build
    unset WASM_INTERFACE_TYPES   
    # Build js-glue version webfoo_lib_bg.wasm
    wasm-pack build --target=web
    popd
    cp_dir wasm/webfoo/pkg htdocs/wasm/webfoo
}

build_xterm(){
    echo "=== Build xterm" >> $BUILD_LOG
    npm install 
    cp_dir node_modules/xterm/dist htdocs/js/xterm
}

build_dev(){
    echo "=== Dev Build" >> $BUILD_LOG
    date >> $BUILD_LOG
}

build_release(){
    echo "=== Release Build" >> $BUILD_LOG
    date >> $BUILD_LOG
    build_xterm
    build_wasm_webfoo
}

build(){
    echo "=== Build " > $BUILD_LOG
    env >> $BUILD_LOG
    env | grep PROFILE >> $BUILD_LOG
    [ $PROFILE == "debug" ] &&  build_dev
    [ $PROFILE == "release" ] &&  build_release
}

usage() {
    cat 1>&2 <<EOF
The script for shTemplate
USAGE:
    bash shTemplate.sh [FLAGS] [OPTIONS]
FLAGS:
    -v, --version           Prints version information
EOF
}

case "$1" in
  build) shift; build $@ ;;
  build-xterm) shift; build_xterm $@ ;;
  build-wasm-webfoo) shift; build_wasm_webfoo $@ ;;
  -h|--help)
      usage
      exit 0
      ;;
  *) usage
     exit 0
     ;;
esac