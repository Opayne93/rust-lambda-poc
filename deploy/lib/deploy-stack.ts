import * as cdk from 'aws-cdk-lib';
import { Construct } from 'constructs';
import { Function, Runtime, FunctionUrlAuthType, Code } from 'aws-cdk-lib/aws-lambda';
import { CfnOutput } from 'aws-cdk-lib';
import * as path from "path";
// import * as sqs from 'aws-cdk-lib/aws-sqs';

export class DeployStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const handler = new Function(this, "opayne-rust-poc", {
      code: Code.fromAsset(path.join(__dirname, "../../target/lambda/rust_lambda_poc/")),
      runtime: Runtime.PROVIDED_AL2,
      handler: "main",
      functionName: "opayne-rust-poc"
    });

    const fnUrl = handler.addFunctionUrl({
      authType: FunctionUrlAuthType.NONE,
    });

    new CfnOutput(this, 'FnUrl', {
      value: fnUrl.url
    })
  }
}

