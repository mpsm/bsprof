#!/usr/bin/env python3

import numpy as np
import os
import shutil
from matplotlib import pyplot as plt


def parse_profile(lines):
    num_cores = []
    build_time = []
    usage_percentage = []
    system_time = []
    user_time = []
    for line in lines:
        if line.startswith("#"):
            num_cores.append(int(line.split("#")[1]))
        elif line.startswith("@"):
            fields = line[1:].split(",")
            build_time.append(float(fields[0]))
            system_time.append(float(fields[1]))
            user_time.append(float(fields[2]))
            usage_percentage.append(int(fields[3].rstrip("%")))

    return num_cores, usage_percentage, build_time, system_time, user_time


def plot_usage(title, filename, cores, usage):
    usage_norm = np.array(usage) / 100.0
    plt.plot(cores, usage_norm, "b-")
    plt.plot(cores, cores, "r--")
    plt.xlabel("Number of cores")
    plt.ylabel("CPU utilization")
    plt.title(title + " / CPU utilization vs. number of cores")
    plt.grid()
    plt.savefig(filename)
    plt.clf()


def plot_build_time(title, filename, cores, build_time, system_time, user_time):
    plt.plot(cores, build_time, "b-", label="Build time")
    plt.plot(cores, system_time, "r-", label="System time")
    plt.plot(cores, user_time, "g-", label="User time")
    plt.plot(
        cores,
        np.array(system_time) + np.array(user_time),
        "y-",
        label="System + User time",
    )
    plt.xlabel("Number of cores")
    plt.ylabel("Build time (s)")
    plt.title(title + " / Build time vs. number of cores")
    plt.grid()
    plt.legend()
    plt.savefig(filename)
    plt.clf()


def main(args):
    if len(args) < 2:
        print("Usage: {} <report file>".format(args[0]))
        return 1

    profile_file = args[1]
    report_dir = "profile_report_" + profile_file.split(".")[1]
    if os.path.exists(report_dir):
        print("Report directory already exists: {}; recreating".format(report_dir))
        shutil.rmtree(report_dir)

    os.mkdir(report_dir)
    with open(profile_file, "r") as f:
        lines = f.readlines()
        title = lines[0].strip()

        cores, usage, build_time, system_time, user_time = parse_profile(lines[1:])
        plot_usage(title, report_dir + "/usage.png", cores, usage)
        plot_build_time(
            title, report_dir + "/time.png", cores, build_time, system_time, user_time
        )

    pass


if __name__ == "__main__":
    import sys

    sys.exit(main(sys.argv))
