#!/usr/bin/env python3

import sys
import threading
import time


def run_subprocess(cmd: str) -> tuple[int, float]:
    import subprocess

    start_time = time.time()
    result = subprocess.run(cmd, shell=True).returncode
    end_time = time.time()

    return (result, end_time - start_time)


def profile_thread(thread_event: threading.Event, wait_time: float = 1.0):
    print("Profiling thread started")
    while True:
        thread_event.wait(wait_time)
        if thread_event.is_set():
            print("Profiling thread is exiting")
            break
        print("Profiling thread is running")

    return 0


def main(args):
    if len(args) < 2:
        print("Usage: {} <command> <command args>".format(args[0]))
        return 1

    cmd = " ".join(args[1:])
    print("Executing: {}".format(cmd))

    # create a thread event to close the profiling thread
    thread_exit_event = threading.Event()

    # spawn a thread before running the subprocess
    thread = threading.Thread(target=profile_thread, args=(thread_exit_event,))
    thread.start()

    ret_value, execution_time = run_subprocess(cmd)
    if ret_value != 0:
        print("Error: subprocess returned non-zero exit code: {}".format(ret_value))
    print("Execution time: {:.3f} seconds".format(execution_time))

    thread_exit_event.set()
    thread.join()

    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
