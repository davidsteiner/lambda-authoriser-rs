from aws_cdk import core as cdk
import aws_cdk.aws_lambda as lambda_

REGION = ""
USER_POOL_ID = ""


class CustomAuthoriserStack(cdk.Stack):
    def __init__(self, scope: cdk.Construct, construct_id: str, **kwargs) -> None:
        super().__init__(scope, construct_id, **kwargs)

        lambda_.Function(
            self,
            "AuthoriserLambda",
            description="Custom authoriser lambda",
            function_name="customAuthoriser",
            code=lambda_.Code.from_asset("lambda.zip"),
            handler="doesnt.matter",
            runtime=lambda_.Runtime.PROVIDED_AL2,
            environment={
                "RUST_BACKTRACE": "1",
                "JWKS_URL": f"https://cognito-idp.{REGION}.amazonaws.com/{USER_POOL_ID}/.well-known/jwks.json"
            },
        )