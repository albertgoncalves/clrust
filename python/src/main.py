#!/usr/bin/env python3

from os import environ
from sys import argv

from matplotlib.pyplot import close, cm, savefig, subplots, tight_layout
from pandas import read_csv

WD = environ["WD"]


def main():
    if 1 < len(argv):
        points = read_csv(argv[1])
        cmap = cm.get_cmap(
            "tab20",
            1 + points.label.max() - points.label.min(),
        )
        points["color"] = points.label.map(lambda label: cmap(label))
        _, ax = subplots()
        ax.scatter(points.x, points.y, c=points.color)
        tight_layout()
        savefig("{}/python/out/output.png".format(WD))
        close()


if __name__ == "__main__":
    main()
