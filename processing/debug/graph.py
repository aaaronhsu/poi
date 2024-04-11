# open points.txt and graph the tuples in matplotlib
import matplotlib.pyplot as plt

with open("poi.csv", "r") as file:
    lines = file.readlines()
    lines = [line.strip() for line in lines]
    lines = [line.split(",") for line in lines]

    x = [float(line[0]) for line in lines]
    y = [float(line[1]) for line in lines]

    plt.scatter(x, y, color="red")

with open("hand.csv", "r") as file:
    lines = file.readlines()
    lines = [line.strip() for line in lines]
    lines = [line.split(",") for line in lines]

    x = [float(line[0]) for line in lines]
    y = [float(line[1]) for line in lines]

    plt.scatter(x, y, color="blue")

plt.legend(["poi", "hand"])
plt.show()