# open points.txt and graph the tuples in matplotlib
import matplotlib.pyplot as plt
import sys


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

        with open(f"points/{t}_poi.csv", "r") as file:
            lines = file.readlines()
            lines = [line.strip() for line in lines]
            lines = [line.split(",") for line in lines]

            x = [float(line[0]) for line in lines]
            y = [float(line[1]) for line in lines]

            plt.scatter(x, y, label=f"{t}_poi")

    with open("points/antispin.csv", "r") as file:
        lines = file.readlines()
        lines = [line.strip() for line in lines]
        lines = [line.split(",") for line in lines]

        x = [float(line[0]) for line in lines]
        y = [float(line[1]) for line in lines]

        plt.scatter(x, y, label="antispin")

    plt.legend()

else:
    for i in range(1, len(sys.argv)):
        with open(f"points/{sys.argv[i]}.csv", "r") as file:
            lines = file.readlines()
            lines = [line.strip() for line in lines]
            lines = [line.split(",") for line in lines]

            x = [float(line[0]) for line in lines]
            y = [float(line[1]) for line in lines]

            plt.scatter(x, y)

    plt.legend(sys.argv[1:])
plt.show()
