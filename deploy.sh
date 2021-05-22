#!/bin/bash
cargo build --release --target x86_64-unknown-linux-musl
cd infrastructure || exit
cp ../target/x86_64-unknown-linux-musl/release/experimental_authoriser ./bootstrap && zip lambda.zip ./bootstrap && rm ./bootstrap
cdk deploy