# nbtedit

**nbtedit** is a lightweight NBT editor. Its main use is editing Minecraft save data.

While for now it only supports editing `level.dat` files, it is planned to support other NBT files.

> [!CAUTION]
> This project is in alpha state. While most of basic functionality works,
> there are many missing features and potential issues.
>
> Make sure to backup saves before editing. I am not responsible
> for potential data loss.

## Downloads
There are available Windows and Linux versions of nbtedit. Go to *Releases* section and download
version for appropriate operating system. Note that these are not installers, but
ready to run versions (there might be required additional dependencies, especially on Unix systems).

> [!NOTE]
> macOS version is not available to download. It is still possible to build one by themself.

## Building
If you do not have Rust installed yet, grab it from [Rust page](https://www.rust-lang.org/tools/install).

After you have downloaded the source code, open terminal, `cd` into source code directory, and run
```sh
cargo build
```

## Licence
This project is licensed under the BSD 3-Clause Licence.
