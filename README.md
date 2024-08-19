# Strikes
## Installation
Currently only the binaries are available. You can pick the release for your platform from the [releases page](https://github.com/tbsklg/strikes/releases).
The binary is a standalone executable and does not require any installation.

Example for the Linux binary:
### Linux
```bash
tar -xvzf strikes-0.0.1-alpha-x86_64-unknown-linux-musl.tar.gz -C <path-to-install>
```

After extracting the binary, you may need to add the path to your PATH environment variable.

## Usage
- Add a strike to a user
```bash
strikes strike <user>
```

- List all strikes
```bash
strikes ls
```

## Use locally only
You can use the local client without a remote server.
It will generate a JSON file where the strikes are stored. 
The default path is in your home directory at '.strikes/db.json'.
You can configure a different location by using the '--db-path' argument or by providing a configuration file.
The argument has precedence over the configuration file.

### Configuration file
The configuration file needs to be a yaml file.

```yaml
local:
    db_path: /path/to/db.json
```

The following command will create a database (db.json) in the current directory.

```bash
strikes --db-path ./my-db.json strike <user>
```

