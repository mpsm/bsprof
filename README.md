# BSPROF a simple build system profiler

`bsprof` allows to wrap a build system command and profile its execution.

    Usage: bsprof [OPTIONS] <command> [args]...

    Arguments:
    <command>  Build command
    [args]...  Additional build arguments

    Options:
    -i, --interval <interval_ms>       Interval in ms between data points [default: 1000]
    -w, --warmup <warmup_ms>           Warmup time in ms [default: 0]
    -c, --cooldown <cooldown_ms>       Cooldown time in ms [default: 0]
    -j, --jobs <jobs>                  Number of jobs
    -s, --sequence                     Profile build system with increasing number of jobs
    -t, --target <target>              Target to build [default: all]
    -C, --clean-target <clean_target>  Target to clean [default: clean]
    -h, --help
    -V, --version                      Print version

Reports are saved as `report.json` in the current directory. You can use `tools/genreport/genreport.py` script to plot the data.
