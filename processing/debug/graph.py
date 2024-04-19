# open points.txt and graph the tuples in matplotlib
import matplotlib.pyplot as plt
import sys


def read_points(file_path):
    with open(file_path, "r") as file:
        lines = file.readlines()
        lines = [line.strip() for line in lines]
        lines = [line.split(",") for line in lines]

        x = [float(line[0]) for line in lines]
        y = [float(line[1]) for line in lines]

        return x, y


if len(sys.argv) < 2:
    types = ["antispin_r", "antispin_g", "inspin_r", "inspin_g"]
    for t in types:
        # with open(f"{t}_hand.csv", "r") as file:
        #     lines = file.readlines()
        #     lines = [line.strip() for line in lines]
        #     lines = [line.split(",") for line in lines]

        #     x = [float(line[0]) for line in lines]
        #     y = [float(line[1]) for line in lines]

        #     plt.scatter(x, y, label=f"{t}_hand")

        x, y = read_points(f"points/{t}_poi.csv")
        plt.scatter(x, y, label=f"{t}_poi")

    x, y = read_points("points/antispin_poi.csv")
    plt.scatter(x, y, label="antispin_poi")

    plt.legend()

else:
    if sys.argv[1] == "steps":
        target = sys.argv[2]  # either hand or poi
        num_steps = int(sys.argv[3])

        for i in range(num_steps):
            if i != 0 and i != num_steps - 1:
                continue
            print(f"fetching points/steps/{target}{i}.csv")
            x, y = read_points(f"points/steps/{target}{i}.csv")
            plt.scatter(x, y, label=f"{target}_step_{i}")

        x, y = read_points(f"points/sim_antispin.csv")
        x = [i / max(x) for i in x]
        y = [i / max(y) for i in y]
        plt.scatter(x, y, label="sim_antispin")
        plt.legend()
    else:
        for i in range(1, len(sys.argv)):
            x, y = read_points(f"points/{sys.argv[i]}.csv")

            plt.scatter(x, y, label=sys.argv[i])

        plt.legend(sys.argv[1:])
plt.show()
