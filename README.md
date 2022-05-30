# rssh - simple unix shell interpreter written in rust

### Installation
rssh runtime dependencies:
    - rust compiler
    - cargo (default rust build system)
    - POSIX compatible operating system. Windows is not supported!

To install rssh type

    cargo install --path .
    strip <path to rssh binary. Usually it will be placed in /usr/local/bin/rssh or ~/.cargo/bin/rssh, when rust is installed via rustup>

### Running rssh
To run new compiled shell just type

    rssh

### Uninstallation
If you decide to uninstall rssh, go to source code root directory and type

    cargo uninstall

