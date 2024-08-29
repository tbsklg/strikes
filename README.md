# Strikes
## Installation

On macOS using homebrew:
```bash
brew tap tbsklg/strikes
brew install strikes
```

There are specific binaries available. You can choose the release for your platform from the [releases page](https://github.com/tbsklg/strikes/releases).
The binary is a standalone executable and does not require installation.

Example for the Linux binary:
### Linux
```bash
tar -xvzf strikes-0.0.1-alpha-x86_64-unknown-linux-musl.tar.gz -C <path-to-install>
```

After extracting the binary, you may need to add the path to your PATH environment variable.

## Usage
```bash
Simple CLI tool to track and assign strikes

Usage: strikes [OPTIONS] [COMMAND]

Commands:
  strike        Add a strike
  ls            List all strikes
  clear         Clear strikes
  check-health  Check health of the client
  help          Print this message or the help of the given subcommand(s)

Options:
  -c, --config-path <CONFIG_PATH>
          Specify the path to the configuration file

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Use with a remote server
You can use a remote server to store the strikes. Either you get access to an existing server or you can deploy the infractructure to your AWS account yourself.
Anyways you need to provide the URL to the server and an API key.

## Use locally
You can use the local client without a remote server.
It will generate a JSON file where the strikes are stored. 

## Configuration file
The default path is in your home directory at '.strikes/db.json'.
You can configure a different location by using the '--db-path' argument or by providing a configuration file.
The argument has precedence over the configuration file.

If you configure a remote and a local server, the remote server will be used. To use the local server, you need to remove the remote configuration from the configuration file. Following is an example of a configuration file:

```yaml
remote:
    base_rul: "https://strikes.example.com"
    api_key: "your-api-key"
local:
    db_path: "/path/to/db.json"
```

You can provide the configuration file by using the '--config-path' argument.
```bash
strikes --config-path /path/to/configuration.yaml strike guenther
```

## Development
### Pre-requisites
You'll need to install:
- [Rust](https://www.rust-lang.org/tools/install) 
- [Docker](https://docs.docker.com/get-docker)
- [Terraform](https://learn.hashicorp.com/tutorials/terraform/install-cli)
- [AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/install-cliv2.html)

### Deploy infrastructure to your AWS account
First of all you need to create a S3 bucket to store the terraform state. Navigate to the infrastructure/remomte-state directory and run:
```bash
terraform init
terraform plan
terraform apply
```
This will create a S3 bucket and a DynamoDB table to store the terraform state. Afterwards you can deploy the infrastructure by navigating to the infrastructure directory and running:

```bash
terraform init
terraform plan
terraform apply
```

### How to test the cli-client
Navigate to cli-client and run:
```bash
cargo test
```

### How to test the infrastructure lambdas
Navigate to infrastructure/lambdas/tests and run:

```bash
docker-compose up -d
```

This will set up a DynamoDB local instance. Afterwards navigate back to infrastructure/lambdas and run:


```bash
cargo test
```
