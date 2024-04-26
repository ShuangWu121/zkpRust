#!/bin/sh

prefix=/home/shuang/zkvm/zkpRust/semaphore/target/debug/build/jemalloc-sys-37c24aa0a3ddb4c8/out
exec_prefix=/home/shuang/zkvm/zkpRust/semaphore/target/debug/build/jemalloc-sys-37c24aa0a3ddb4c8/out
libdir=${exec_prefix}/lib

LD_PRELOAD=${libdir}/libjemalloc.so.2
export LD_PRELOAD
exec "$@"
