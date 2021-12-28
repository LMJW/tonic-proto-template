# tonic-proto-template

This is simple tonic repo with proto buf build

## How to setup

1. add the repository contain "proto" as a submodule

```
git add submodule git@github.com:hyperium/tonic.git
```

2. create a `src/protogen` directory and `build.rs`, the `protogen` directory is used to store the generated code. See `build.rs` to see how to generate the code.

3. in the `lib.rs`, define the module so we can import the generated code in the actual code we written.
