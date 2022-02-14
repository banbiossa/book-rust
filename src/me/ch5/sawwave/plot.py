#!/usr/bin/env python3
import numpy as np, sys
import matplotlib.pyplot as plt
import fire


def main(filename: str):
    x, y = ([], [])
    lines = open(filename, "r").read().split("\n")
    for i, v in enumerate(lines):
        if v != "":
            x.append(i)
            y.append(float(v))
        if i > 1500:
            break
    plt.plot(x, y)
    plt.show()


if __name__ == '__main__':
    fire.Fire(main)