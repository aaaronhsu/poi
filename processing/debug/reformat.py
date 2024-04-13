import matplotlib.pyplot as plt
import csv

with open("points/poi_short.csv", "r") as f:
    data = f.readlines()
    data = [line.strip() for line in data]
    data = [line.split(",") for line in data]

    x = [float(line[0]) for line in data]
    y = [float(line[1]) for line in data]

    plt.scatter(x, y)
    plt.show()

    data_out = [["id", "type", "frame_num", "x", "y", "confidence"]]

    for i, row in enumerate(data):
        x = float(row[0])
        y = float(row[1])
        data_out.append([0, 0, i, x, y, 1])

    with open("poi_short.csv", "w", newline="") as file:
        writer = csv.writer(file)
        writer.writerows(data_out)
