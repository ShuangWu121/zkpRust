#!/bin/sh

prefix=/home/shuang/zkvm/zkpRust/semaphore/target/release/build/jemalloc-sys-02ccacc8eb4a779e/out
exec_prefix=/home/shuang/zkvm/zkpRust/semaphore/target/release/build/jemalloc-sys-02ccacc8eb4a779e/out
libdir=${exec_prefix}/lib

LD_PRELOAD=${libdir}/libjemalloc.so.2
export LD_PRELOAD
exec "$@"
