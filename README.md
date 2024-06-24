# rplugin

`gpbackup` plugin that behaves exactly like `example_plugin.bash`, but is a
native executable that does not use any coreutils. Take a look inside
`Cargo.toml` for provided features.

### Usage

```sh
$ cargo build --release --features <...>
...
$ ./generate_config.sh
...
$ gprestore/gpbackup --plugin-config $PWD/rplugin_config.yaml <...>
```
