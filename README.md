# ArceOS

[![CI](https://github.com/rcore-os/arceos/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/rcore-os/arceos/actions)

An experimental modular operating system (or unikernel) written in Rust.

ArceOS was inspired a lot by [Unikraft](https://github.com/unikraft/unikraft).

🚧 Working In Progress.

## Features & TODOs

* [x] Architecture: riscv64, aarch64
* [x] Platform: QEMU virt riscv64/aarch64
* [x] Multi-thread
* [x] Cooperative FIFO scheduler
* [x] VirtIO net/blk drivers
* [x] TCP net stack using [smoltcp](https://github.com/smoltcp-rs/smoltcp)
* [x] Synchronization/Mutex
* [x] Kernel preemption
* [ ] File system
* [ ] Compatible with Linux apps
* [ ] Interrupt driven device I/O
* [ ] Async I/O
* [ ] SMP

## Example apps

Example applications can be found in the [apps/](apps/) directory. All applications must at least depend on the following modules, while other modules are optional:

* [axruntime](modules/axruntime/): Bootstraping from the bare-metal environment, and initialization.
* [axhal](modules/axhal/): Hardware abstraction layer, provides unified APIs for cross-platform.
* [axconfig](modules/axconfig/): Platform constants and kernel parameters, such as physical memory base, kernel load addresses, stack size, etc.
* [axlog](modules/axlog/): Multi-level log definition and printing.
* [axerror](modules/axerror/): Error code definition.

The currently supported applications (Rust), as well as their dependent modules and features, are shown in the following table:

| App | Extra modules | Enabled features | Description |
|-|-|-|-|
| [helloworld](apps/helloworld/) | | | A minimal app that just prints a string |
| [exception](apps/exception/) | | paging | Exception handling test |
| [memtest](apps/memtest/) | axalloc | alloc, paging | Dynamic memory allocation test |
| [yield](apps/task/yield/) | axalloc, axtask | alloc, paging, multitask, sched_fifo | Multi-threaded yielding test |
| [parallel](apps/task/parallel/) | axalloc, axtask | alloc, paging, multitask, sched_fifo | Parallel computing test (to test synchronization & mutex) |
| [sleep](apps/task/sleep/) | axalloc, axtask | alloc, paging, multitask, sched_fifo | Thread sleeping test |
| [httpclient](apps/net/httpclient/) | axalloc, axdriver, axnet | alloc, paging, net | A simple client that sends an HTTP request and then prints the response |
| [echoserver](apps/net/echoserver/) | axalloc, axdriver, axnet, axtask | alloc, paging, net, multitask | A multi-threaded TCP server that reverses messages sent by the client  |
| [httpserver](apps/net/httpserver/) | axalloc, axdriver, axnet, axtask | alloc, paging, net, multitask | A multi-threaded HTTP server that serves a static web page |

## Build & Run

### Example apps

```bash
# in arceos directory
make A=path/to/app ARCH=<arch> LOG=<log> NET=[y|n] FS=[y|n]
```

Where `<arch>` should be one of `riscv64`, `aarch64`.

`<log>` should be one of `off`, `error`, `warn`, `info`, `debug`, `trace`.

`path/to/app` is the relative path to the example application.

More arguments and targets can be found in [Makefile](Makefile).

For example, to run the [httpserver](apps/net/httpserver/) on `qemu-system-aarch64` with 4 cores:

```bash
make A=apps/net/httpserver ARCH=aarch64 LOG=info NET=y SMP=4 run
```

### You custom apps

#### Rust

1. Create a new rust package with `no_std` and `no_main` environment.
2. Add the `libax` dependency to `Cargo.toml`:

    ```toml
    [dependencies]
    libax = { path = "/path/to/arceos/ulib/libax", features = ["..."] }
    ```

3. Call library functions from `libax` in your code, like the [helloworld](apps/helloworld/) example.
4. Build your application with ArceOS, by running the `make` command in the application directory:

    ```bash
    # in app directory
    make -C /path/to/arceos A=$(pwd) ARCH=<arch> run
    ```

    All arguments and targets are the same as above.

#### C

1. Create a `axbuild.mk` file in your project:

    ```
    app/
    ├── foo.c
    ├── bar.c
    └── axbuild.mk
    ```

2. Add build targets to `axbuild.mk` (see [this](apps/c/sqlite3/axbuild.mk) file for more advanced usage):

    ```Makefile
    # in axbuild.mk
    app-objs := foo.o bar.o
    ```

3. Build your application with ArceOS, by running the `make` command in the application directory:

    ```bash
    # in app directory
    make -C /path/to/arceos A=$(pwd) ARCH=<arch> run
    ```

## Design

![](doc/ArceOS.svg)
