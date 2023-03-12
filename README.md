# BSPROF a simple build system profiler

`bsprof` allows to wrap a build system command and profile its execution.

    Usage: bsprof [OPTIONS] <command> [args]...

    Arguments:
    <command>  Command to run
    [args]...  Command arguments

    Options:
    -i, --interval <interval_ms>  Interval in ms between data points [default: 1000]
    -w, --warmup <warmup_ms>      Warmup time in ms [default: 0]
    -c, --cooldown <cooldown_ms>  Cooldown time in ms [default: 0]
    -h, --help                    Print help

Reports are saved as `report.json` in the current directory. You can use `tools/genreport/genreport.py` script to plot the data.
