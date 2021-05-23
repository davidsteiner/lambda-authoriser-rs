Rust Request Authoriser Blueprint
=================================

This repository contains the code for a custom authoriser lambda function
written in Rust, using CDK written in Python to describe the infrastructure.

I use this to authenticate WebSocket API gateways with a Cognito user pool,
so the implementation is intended for that. It should be straight-forward
to adapt it to other use-cases, though.

The instruction belows are for Mac OS.

Install Cross-Compilation Tools
-------------------------------

#. Follow the steps here to install the linker: `aws-lambda-rust-runtime <https://github.com/awslabs/aws-lambda-rust-runtime#aws-cli>`_

#. Create symlink for musl-gcc

.. code::

   ln -s /usr/local/opt/musl-cross/bin/x86_64-linux-musl-gcc /usr/local/bin/musl-gcc
   
Deployment
----------

The deployment requires:

1. CDK CLI (test with 1.105.0)
2. Pip requirements installed
3. The Rust toolchain
4. Cross-compilation tools

Running the deployment:

.. code::

   ./deploy.sh

This uses :code:`cargo` to build the Rust project.
It then copies and zips the executable inside the :code:`./infrastructure` folder,
and runs :code:`cdk deploy` to deploy it on AWS.