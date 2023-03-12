import json
import os

import numpy as np
from matplotlib import pyplot as plt

PLOT_WIDTH = 900
PLOT_HEIGHT = 600
PLOT_DPI = 72


def gen_usage_plot(times, cpu_usage, memory_percent_usage):
    plt.figure(figsize=(PLOT_WIDTH / PLOT_DPI, PLOT_HEIGHT / PLOT_DPI), dpi=PLOT_DPI)
    plt.plot(times, cpu_usage, label="CPU Usage", color="red")
    plt.plot(times, memory_percent_usage, label="Memory Usage", color="blue")
    plt.xlabel("Time (s)")
    plt.ylabel("Usage (%)")
    plt.legend()
    plt.title("CPU and Memory Usage")
    plt.grid(True)
    plt.savefig("cpu_memory_usage.png", dpi=PLOT_DPI)
    plt.close()


def gen_cores_plot(times, cpu_cores_data):
    plt.figure(figsize=(PLOT_WIDTH / PLOT_DPI, PLOT_HEIGHT / PLOT_DPI), dpi=PLOT_DPI)
    for i, cpu_core_data in enumerate(cpu_cores_data):
        plt.plot(times, cpu_core_data, label="CPU Core {}".format(i))
    plt.grid(True)
    plt.legend()
    plt.xlabel("Time (s)")
    plt.ylabel("CPU Cores Utilization (%)")
    plt.title("CPU Cores Utilization ({} cores)".format(len(cpu_cores_data)))
    plt.savefig("cpu_cores_utilization.png")
    plt.close()


def main(args):
    if len(args) < 2:
        print("Usage: python genreport.py <datafile>")
        return 1

    filename = args[1]
    try:
        fd = open(filename, "r")
    except IOError:
        print("Could not open file: %s" % filename)
        return 1

    profile_data = json.load(fd)
    times = np.array([d["elapsed"] for d in profile_data["datapoints"]])
    cpu_usage = np.array([d["cpu_usage"] for d in profile_data["datapoints"]])
    memory_usage = np.array([d["memory_usage"] for d in profile_data["datapoints"]])
    cpus_utilization = np.array(
        [d["cpus_utilization"] for d in profile_data["datapoints"]]
    )
    cpu_cores_data = list(map(np.array, zip(*cpus_utilization)))
    memory_percent_usage = (
        memory_usage / profile_data["system_info"]["total_memory"] * 100
    )

    assert len(times) == len(cpu_usage) == len(memory_usage)

    plots_dir = "plots"
    if not os.path.exists(plots_dir):
        os.mkdir(plots_dir)
    os.chdir(plots_dir)

    gen_usage_plot(times, cpu_usage, memory_percent_usage)
    gen_cores_plot(times, cpu_cores_data)

    return 0


if __name__ == "__main__":
    import sys

    sys.exit(main(sys.argv))
