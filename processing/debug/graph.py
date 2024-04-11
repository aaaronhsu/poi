# open points.txt and graph the tuples in matplotlib
import matplotlib.pyplot as plt
import sys

for i in range(1, len(sys.argv)):
    with open(f"{sys.argv[i]}.csv", "r") as file:
        lines = file.readlines()
        lines = [line.strip() for line in lines]
        lines = [line.split(",") for line in lines]

        x = [float(line[0]) for line in lines]
        y = [float(line[1]) for line in lines]

        plt.scatter(x, y)

plt.legend(sys.argv[1:])
plt.show()
