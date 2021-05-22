#!/usr/bin/env python3
from aws_cdk import core

from stack import CustomAuthoriserStack


app = core.App()
CustomAuthoriserStack(app, "CustomRustAuthoriserStack")

app.synth()
