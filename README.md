# watchdiff
Watch output and trigger on diff!

Ever want to have `watch` output only tell you what changed? And not only what, but when? Now you can! Enter: `watchdiff`.

Watchdiff will monitor command output and will output in the terminal whenever a difference was detected in the output of the running program.

## Usage

```bash
USAGE:
    watchdiff [FLAGS] [OPTIONS] [command]...

FLAGS:
    -h, --help         Prints help information
    -p, --permament    Compare to the initial output (permament mode)
    -V, --version      Prints version information

OPTIONS:
    -d, --delay <delay>    Delay between runs in seconds [default: 2]

ARGS:
    <command>...    Command to run
```

To run a command with arguments use `--` to escape `watchdiff`s argument parser, like so:
```
watchdiff -n 5 -- ls -alh
```
