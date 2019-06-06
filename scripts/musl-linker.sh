#!/bin/bash

export GCC_ROOT="$TOOLCHAIN_ROOT/lib/gcc/x86_64-linux-musl/5.3.0"

args=()
for arg in "$@"; do
    if [[ $arg = *"Bdynamic"* ]]; then
        args+=() # we do not want this arg
    elif [[ $arg = *"crti.o"* ]]; then
        args+=("$arg" "$GCC_ROOT/crtbeginT.o" "-Bstatic")
    elif [[ $arg = *"crtn.o"* ]]; then
        args+=("-lgcc" "-lgcc_eh" "-lc" "$GCC_ROOT/crtend.o" "$arg")
    else
        args+=("$arg")
    fi
done

echo "RUNNING WITH ARGS: ${args[@]}"
musl-gcc "${args[@]}"
