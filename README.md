# nbtedit

**nbtedit** is a cross-platform NBT editor.
It can handle Minecraft world data and generic NBT files.
SNBT import and export is in plans.

> [!WARNING]
> This project is in alpha state. While most of basic functionality works,
> some issues may occur.
>
> Always backup your NBT file before editing.

## Downloads
Releases are available for Windows and Linux. Go to *Releases* section and download
version for operating system of Your choice. Note that these are not installers, but
standalone executable files.

Windows version should not require any additional steps to run.

On Linux X11 server is required. Wayland environment is not tested.

> [!NOTE]
> macOS version is not available to download. It is still possible to build one by themself.

## Building
If you do not have Rust installed yet, grab it from [Rust page](https://www.rust-lang.org/tools/install).

Installing dependencies may be required. Particularly, on Linux
systems, X11 development files are needed (`libx11-dev` on Debian-based
systems, `libx11-devel` on Fedora-based systems or equivalent).

After you have downloaded the source code, open terminal, `cd` into source code directory, and run
```sh
cargo build --release
```

Executable will be in `target/release` directory.

## Licence
This project is licensed under the BSD 3-Clause Licence.
