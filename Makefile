## Synth Lambda Rs
synth-lambda-rs:
	npm --prefix deploy run cdk synth LambdaRS --verbose

## Deploy finderLambdaRs
deploy-lambda-rs:
	npm --prefix deploy run cdk deploy LambdaRS --verbose

## Build artifact-finder-rs
build-lambda-rs:
	cargo lambda build --release --arm64 --output-format zip --manifest-path=./Cargo.toml

# install cdk
install-cdk:
	npm install --prefix deploy

# destory stack
destroy-lambda-rs:
	npm --prefix deploy run cdk destroy LambdaRS --verbose