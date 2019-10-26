#!/usr/bin/env python

# https://github.com/pavankalyan1997/Machine-learning-without-any-libraries/tree/master/2.Clustering/1.K_Means_Clustering

from os import environ
from random import randint, random, seed

from matplotlib.pyplot import close, savefig, subplots, tight_layout
import numpy as np
from pandas import read_csv

WD = environ["WD"]


class KMeans:
    def __init__(self, X, K):
        self.X = X
        self.Output = {}
        self.K = K
        self.M = self.X.shape[0]

    def centroids(self, x, k):
        i = randint(0, x.shape[0])
        t = np.array([x[i]])
        for _ in range(1, k):
            D = np.empty(len(x))
            j = 0
            for j in range(len(x)):
                D[j] = np.min(np.sum((x[j] - t) ** 2))
            prob = D / np.sum(D)
            cummulative_prob = np.cumsum(prob)
            r = random()
            i = 0
            for j, p in enumerate(cummulative_prob):
                if r < p:
                    i = j
                    break
            t = np.append(t, [x[i]], axis=0)
        return t.T

    def fit(self, n):
        self.Centroids = self.centroids(self.X, self.K)
        for _ in range(n):
            distance = np.array([]).reshape(self.M, 0)
            for k in range(self.K):
                distance = np.c_[distance, np.sum(
                    (self.X - self.Centroids[:, k]) ** 2,
                    axis=1,
                )]
            C = np.argmin(distance, axis=1) + 1
            Y = {}
            for k in range(self.K):
                Y[k + 1] = np.array([]).reshape(2, 0)
            for i in range(self.M):
                Y[C[i]] = np.c_[Y[C[i]], self.X[i]]
            for k in range(self.K):
                Y[k + 1] = Y[k + 1].T
            for k in range(self.K):
                if len(Y[k + 1]) != 0:
                    self.Centroids[:, k] = np.mean(Y[k + 1], axis=0)
            self.Output = Y

    def predict(self):
        return (self.Output, self.Centroids.T)

    def sum_error(self):
        x = 0
        for k in range(self.K):
            x += np.sum((self.Output[k + 1] - self.Centroids[:, k]) ** 2)
        return x


def main():
    seed(1)
    x = read_csv("{}/data.csv".format(WD)).iloc[:, [3, 4]].values
    n = 150
    m = 10
    sum_error = np.empty(m)
    ks = []
    for i in range(m):
        k = i + 1
        ks.append(k)
        print(k)
        kmeans = KMeans(x, k)
        kmeans.fit(n)
        (output, centroids) = kmeans.predict()
        sum_error[i] = kmeans.sum_error()
        _, ax = subplots()
        for j in range(k):
            ax.scatter(
                output[j + 1][:, 0],
                output[j + 1][:, 1],
                s=50,
                alpha=0.75,
            )
        ax.scatter(centroids[:, 0], centroids[:, 1], s=300, c="k", marker="X")
        tight_layout()
        savefig("{}/python/out/k_{}.png".format(WD, k))
        close()
    _, ax = subplots()
    ax.plot(ks, sum_error)
    tight_layout()
    savefig("{}/python/out/sum_error.png".format(WD))
    close()


if __name__ == "__main__":
    main()
