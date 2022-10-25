## A Rust Lambda POC

This is a lambda function written in Rust. It accepts an http POST and returns
a simple "hello world" message, depending on what data is supplied.

### Deploying
To deploy this function, you will need the following tools:
- cargo-lambda (https://github.com/cargo-lambda/cargo-lambda)
- okta-aws CLI
- SAM (https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-cli-install-mac.html)

To deploy to the sandbox account, perform the following:
1. Assume the admin role on the sandbox account using `okta-aws`
2. Run `cargo lambda build --release`
   1. This will generate a `bootstrap` file inside the target/lambda/rust_lambda_poc folder. This is the binary file of the actual function
3. Run `sam deploy --config-env sandbox --profile sandbox --capabilities CAPABILITY_IAM`

If the build is successful, the function url should be outputted to the logs. 
Test it out by making an HTTP post, and provide the following body:
```
{"firstName" : "Ferris the Rustacean"}
```