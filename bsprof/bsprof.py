#!/usr/bin/env python3

import sys
import time


def run_subprocess(cmd: str) -> tuple[int, float]:
    import subprocess

    start_time = time.time()
    result = subprocess.run(cmd, shell=True).returncode
    end_time = time.time()

    return (result, end_time - start_time)


def main(args):
    if len(args) < 2:
        print("Usage: {} <command> <command args>".format(args[0]))
        return 1

    cmd = " ".join(args[1:])
    print("Executing: {}".format(cmd))

    ret_value, execution_time = run_subprocess(cmd)
    if ret_value != 0:
        print("Error: subprocess returned non-zero exit code: {}".format(ret_value))
    print("Execution time: {:.3f} seconds".format(execution_time))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
