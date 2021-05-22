# Install Cross-Compilation Tools

1. Follow steps here to install the linker: https://github.com/awslabs/aws-lambda-rust-runtime#aws-cli
2. Create symlink for musl-gcc
   > ln -s /usr/local/opt/musl-cross/bin/x86_64-linux-musl-gcc /usr/local/bin/musl-gcc
   
# Deployment
   > ./deploy.sh

The deployment requires:
1. CDK CLI
2. Pip requirements installed
3. The Rust toolchain
4. Cross-compilation tools
