import json
import os

import numpy as np
from matplotlib import pyplot as plt

PLOT_WIDTH = 900
PLOT_HEIGHT = 600
PLOT_DPI = 72


def gen_usage_plot(times, cpu_usage, memory_percent_usage, chart_id):
    plt.figure(figsize=(PLOT_WIDTH / PLOT_DPI, PLOT_HEIGHT / PLOT_DPI), dpi=PLOT_DPI)
    plt.plot(times, cpu_usage, label="CPU Usage", color="red")
    plt.plot(times, memory_percent_usage, label="Memory Usage", color="blue")
    plt.xlabel("Time (s)")
    plt.ylabel("Usage (%)")
    plt.legend()
    plt.title("CPU and Memory Usage, {} jobs".format(chart_id))
    plt.grid(True)
    plt.savefig("cpu_memory_usage_{:02d}.png".format(chart_id), dpi=PLOT_DPI)
    plt.close()


def gen_cores_plot(times, cpu_cores_data, chart_id):
    plt.figure(figsize=(PLOT_WIDTH / PLOT_DPI, PLOT_HEIGHT / PLOT_DPI), dpi=PLOT_DPI)
    for i, cpu_core_data in enumerate(cpu_cores_data):
        plt.plot(times, cpu_core_data, label="CPU Core {}".format(i))
    plt.grid(True)
    plt.xlabel("Time (s)")
    plt.ylabel("CPU Cores Utilization (%)")
    plt.title(
        "CPU Cores Utilization ({} cores, {} jobs)".format(
            len(cpu_cores_data), chart_id
        )
    )
    plt.savefig("cpu_cores_utilization_{:02d}.png".format(chart_id), dpi=PLOT_DPI)
    plt.close()


def gen_time_plot(jobs, elapsed, user, system):
    plt.figure(figsize=(PLOT_WIDTH / PLOT_DPI, PLOT_HEIGHT / PLOT_DPI), dpi=PLOT_DPI)
    plt.plot(jobs, elapsed, label="Elapsed Time", color="red")
    plt.plot(jobs, user, label="User Time", color="blue")
    plt.plot(jobs, system, label="System Time", color="green")
    plt.plot(jobs, user + system, label="User + System Time", color="orange")
    plt.xlabel("Jobs")
    plt.ylabel("Time (s)")
    plt.legend()
    plt.title("Time")
    plt.grid(True)
    plt.savefig("time.png", dpi=PLOT_DPI)
    plt.close()


def gen_build_time_plot(jobs, elapsed):
    plt.figure(figsize=(PLOT_WIDTH / PLOT_DPI, PLOT_HEIGHT / PLOT_DPI), dpi=PLOT_DPI)
    plt.plot(jobs, elapsed, label="Elapsed Time", color="red")
    plt.xlabel("Jobs")
    plt.ylabel("Time (s)")
    plt.legend()
    plt.title("Build Time vs jobs")
    plt.grid(True)
    plt.savefig("build_time.png", dpi=PLOT_DPI)
    plt.close()


def gen_cpu_time_plot(jobs, elapsed_time, user_time, system_time, cores):
    cpu_time = (user_time + system_time) / elapsed_time
    plt.figure(figsize=(PLOT_WIDTH / PLOT_DPI, PLOT_HEIGHT / PLOT_DPI), dpi=PLOT_DPI)
    plt.plot(jobs, jobs, label="Reference CPU utilization", color="red", linestyle="--")
    plt.plot(jobs, cpu_time, label="Effective CPU utilization", color="blue")
    plt.xlabel("Jobs")
    plt.ylabel("Cores used")
    plt.legend()
    plt.title("Effective CPU utilization ({} cores)".format(cores))
    plt.grid(True)
    plt.savefig("cpu_time.png", dpi=PLOT_DPI)
    plt.close()


def rusage_json_time_to_float(rusage_time):
    return float(rusage_time["secs"]) + float(rusage_time["nanos"]) / 1_000_000_000.0


def main(args):
    if len(args) < 2:
        print("Usage: python genreport.py <datafile> [cores]")
        print("")
        print("\t<datafile> - path to the data file")
        print(
            "\t[charts] - comma separated list of cores to generate charts (default: max cores used)"
        )
        return 1

    filename = args[1]
    try:
        fd = open(filename, "r")
    except IOError:
        print("Could not open file: %s" % filename)
        return 1

    charts = [int(a) for a in args[2].split(",")] if len(args) > 2 else None

    report = json.load(fd)

    plots_dir = "plots"
    if not os.path.exists(plots_dir):
        os.mkdir(plots_dir)
    os.chdir(plots_dir)

    # use the last profile data if charts is not specified
    if charts is None:
        charts = [report["profile_results"][-1]["jobs"]]

    for profile_data in report["profile_results"]:
        jobs = profile_data["jobs"]

        if jobs not in charts:
            continue

        times = np.array([d["elapsed"] for d in profile_data["datapoints"]])
        cpu_usage = np.array([d["cpu_usage"] for d in profile_data["datapoints"]])
        memory_usage = np.array([d["memory_usage"] for d in profile_data["datapoints"]])
        cpus_utilization = np.array(
            [d["cpus_utilization"] for d in profile_data["datapoints"]]
        )
        cpu_cores_data = list(map(np.array, zip(*cpus_utilization)))
        memory_percent_usage = (
            memory_usage / report["system_info"]["total_memory"] * 100
        )

        assert len(times) == len(cpu_usage) == len(memory_usage)

        gen_usage_plot(times, cpu_usage, memory_percent_usage, jobs)
        gen_cores_plot(times, cpu_cores_data, jobs)

    jobs = np.array([d["jobs"] for d in report["profile_results"]])
    elapsed_time = np.array([d["elapsed_time"] for d in report["profile_results"]])
    user_time = np.array(
        [
            rusage_json_time_to_float(d["rusage"]["user_time"])
            for d in report["profile_results"]
        ]
    )
    system_time = np.array(
        [
            rusage_json_time_to_float(d["rusage"]["system_time"])
            for d in report["profile_results"]
        ]
    )

    cores = report["system_info"]["num_cpus"]

    gen_time_plot(jobs, elapsed_time, user_time, system_time)
    gen_build_time_plot(jobs, elapsed_time)
    gen_cpu_time_plot(jobs, elapsed_time, user_time, system_time, cores)

    return 0


if __name__ == "__main__":
    import sys

    sys.exit(main(sys.argv))
