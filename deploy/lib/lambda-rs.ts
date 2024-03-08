import * as cdk from 'aws-cdk-lib';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import { Construct } from 'constructs';


export class LambdaRS extends cdk.Stack {
    constructor(scope: Construct, id: string, props?: cdk.StackProps) {
        super(scope, id, props);

        const rustLambda = new lambda.Function(this, 'LambdaRS', {
            runtime: lambda.Runtime.PROVIDED_AL2023,
            code: lambda.Code.fromAsset('../target/lambda/lambda-rust-cdk/bootstrap.zip'),
            handler: 'bootstrap',
            memorySize: 128,
            timeout: cdk.Duration.seconds(5),
            architecture: lambda.Architecture.ARM_64, // Optional, if needed
        });

        const fnUrl = rustLambda.addFunctionUrl({
            authType: lambda.FunctionUrlAuthType.NONE,
          });
      
          new cdk.CfnOutput(this, 'LambdaFunctionUrl', {
            description: 'Url of the lambda function',
            value: fnUrl.url,
          });

    }
}
