# Strikes

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
You can configure a different location by using the '--db_path' argument.

The following command will create a database (db.json) in the current directory.

```bash
strikes --db_path . strike guenther
```

