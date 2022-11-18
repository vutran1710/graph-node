# Scripts

## `release.sh`

1. Checks that all workspace crates use the same version via `version.workspace = true`.
2. Updates the version in the root `Cargo.toml` as indicated by the user: `major`, `minor`, or `patch`.
3. Updates `Cargo.lock` via `cargo check --tests`.
4. Adds the changes in a `Release vX.Y.Z` commit.

Upon failure, the script will print some kind of error message and stop before committing the changes.

### Usage

The only argument it accepts is the type of release you want to do.

```bash
# E.g. you're on v0.28.0 and must relese v0.28.1.
$ ./scripts/release.sh patch
```

Example output:

```
Current version: "0.25.1"
New version: "0.25.2"
Changing 18 toml files
Toml files are still consistent in their version after the update
Updating Cargo.lock file
    Finished dev [unoptimized + debuginfo] target(s) in 0.58s
Cargo.lock file updated
Updating version of the Cargo.{lock, toml} files succeded!
[otavio/update-news-0-25-2 2f2175bae] Release 0.25.2
 20 files changed, 38 insertions(+), 38 deletions(-)
```

This script contains several assertions to make sure no mistake has been made. Unfortunately for now we don't have a way to revert it, or to recover from an error when it fails in the middle of it, this can be improved in the future.

```
cargo run -p graph-node --release -- \
--postgres-url postgresql://graph-node:let-me-in@localhost:5432/graph-node \
--ethereum-rpc polygon:archive:https://polygon-mainnet.g.alchemy.com/v2/NopMECFaL2nSYsCRflZ9dOTNxWV3BZy- \
--bus-url amqp://guest:guest@localhost:5672 \
--ipfs 127.0.0.1:5001
```
