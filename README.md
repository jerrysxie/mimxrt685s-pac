# MIMXRT685s Peripheral Access Crate

This crate provides an autogenerated API for access to NXP MIMXRT685s
peripherals. The API is generated using
[svd2rust](https://github.com/rust-embedded/svd2rust).

## Regenerating the PAC

On a unix-style OS, all you need are these commands:

```console
$ svdtools patch patch/MIMXRT685S_cm33.yaml
$ svd2rust -i svd/MIMXRT685S_cm33.svd.patched --reexport-interrupt --ignore-groups --impl-defmt defmt --impl-debug --impl-debug-feature debug
$ rm -r src/*
$ form -i lib.rs -o src
$ rm lib.rs
$ cargo fmt
```

On windows you need to replace the `/` with `\` and additionally run
`dos2unix` to convert the line endings, like so:

```console
$ svdtools.exe patch patch/MIMXRT685S_cm33.yaml
$ svd2rust.exe -i svd\MIMXRT685S_cm33.svd.patched --reexport-interrupt --ignore-groups --impl-defmt defmt --impl-debug --impl-debug-feature debug
$ rm -r src\*
$ form -i lib.rs -o src
$ rm lib.rs
$ cargo fmt
$ cd src
$ dos2unix **\*.rs *.rs
```



