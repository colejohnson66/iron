# Iron

Iron is a new web browser.

## Notice

Iron is developed and tested on an [Arch Linux](https://www.archlinux.org/) installation.
It may not compile, let alone run, at all on macOS or Windows.

## Building and Running

In order to run a copy, you must have [CMake](https://cmake.org/) installed.
Then you can run the following:

```bash
> git clone --recurse-submodules https://github.com/colejohnson66/iron.git
> cd iron
> cmake -S . -B build
> cmake --build build
> ./build/iron
```

## License

Iron is licensed under [GNU General Public License 3.0](https://www.gnu.org/licenses/gpl-3.0.en.html) or later.
