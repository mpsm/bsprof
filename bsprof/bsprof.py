#!/usr/bin/env python3

import os
import sys
import threading
import time
from dataclasses import dataclass

import psutil


@dataclass
class ProfileData:
    record_time: float
    os_load: tuple[float, float, float]
    core_load: list[float]


def get_current_profile_data() -> ProfileData:
    data = ProfileData(
        record_time=time.time(),
        os_load=os.getloadavg(),
        core_load=psutil.cpu_percent(percpu=True),  # type: ignore
    )
    return data


def run_subprocess(cmd: str) -> tuple[int, float]:
    import subprocess

    start_time = time.time()
    result = subprocess.run(cmd, shell=True).returncode
    end_time = time.time()

    return (result, end_time - start_time)


def profile_thread(
    profiling_data: list[ProfileData],
    thread_event: threading.Event,
    wait_time: float = 1.0,
):
    while True:
        profiling_data.append(get_current_profile_data())
        thread_event.wait(wait_time)
        if thread_event.is_set():
            break

    return 0


def process_profiling_data(profiling_data: list[ProfileData]):
    print("Records: {}".format(len(profiling_data)))
    print(profiling_data)


def main(args):
    if len(args) < 2:
        print("Usage: {} <command> <command args>".format(args[0]))
        return 1

    cmd = " ".join(args[1:])
    print("Executing: {}".format(cmd))

    # create a thread event to close the profiling thread
    thread_exit_event = threading.Event()

    # profiling data
    profiling_data = []

    # spawn a thread before running the subprocess
    thread = threading.Thread(
        target=profile_thread,
        args=(
            profiling_data,
            thread_exit_event,
        ),
    )
    thread.start()

    ret_value, execution_time = run_subprocess(cmd)
    if ret_value != 0:
        print("Error: subprocess returned non-zero exit code: {}".format(ret_value))
    print("Execution time: {:.3f} seconds".format(execution_time))
    print("Execution stats:")
    process_profiling_data(profiling_data)

    thread_exit_event.set()
    thread.join()

    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
