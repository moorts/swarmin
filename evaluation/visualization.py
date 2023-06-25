import json

import matplotlib.pyplot as plt
import matplotlib.colors as mcolors
import numpy as np

from objective_functions import *


def parse_history(history):
    out = []
    iterates = [list(map(lambda x: x["position"], iterate["particles"])) for iterate in history]
    for iterate in history:
        particles = list(map(lambda x: x["position"], iterate["particles"]))
        velocities = list(map(lambda x: x["position"], iterate["velocities"]))
        particle_bests = list(map(lambda x: x[0]["position"], iterate["particle_bests"]))
        global_best_x = iterate["global_best_x"]["position"]
        global_best_value = iterate["global_best_value"]

        out.append({
            "particles": particles,
            "velocities": velocities,
            "particle_bests": particle_bests,
            "global_best_x": global_best_x,
            "global_best_value": global_best_value,
        })
        
    return out

def parse_2d_result(result):
    out = {}

    # History
    out["history"] = parse_history(result["history"])

    # Parameters
    xlims = (result["lower_bound"]["position"][0], result["upper_bound"]["position"][0])
    ylims = (result["lower_bound"]["position"][1], result["upper_bound"]["position"][1])
    out["xlims"] = xlims
    out["ylims"] = ylims

    return out


def plot_2d_history(f, history, xlims, ylims):
    fig, ax = plt.subplots()

    x_disc = np.linspace(xlims[0], xlims[1], num = 351)
    y_disc = np.linspace(ylims[0], ylims[1], num = 351)
    X, Y = np.meshgrid(x_disc, y_disc)

    # Get the function values of f on the grid for contour plots
    Z = np.zeros(X.shape)
    for i, x_comp in enumerate(x_disc):
        for j, y_comp in enumerate(y_disc):
            Z[i,j] = f(np.array([x_comp,y_comp]))

    contour_vals = np.linspace(np.min(Z)-0.2*np.abs(np.min(Z)), np.max(Z)+0.2*np.abs(np.max(Z)), 20)

    values = [iterate["global_best_x"] for iterate in history]

    plt.contourf(X,Y,Z.T, cmap='gray_r', levels = contour_vals)

    print(values)
    xs, ys = zip(*values)

    plt.plot(xs, ys)

    plt.gca().set_aspect('equal', 'box')
    plt.title("PSO: Global Best Values")

    ax.set_xlim(xlims)
    ax.set_ylim(ylims)

    return fig

with open('../pso_result.json') as f:
    result = parse_2d_result(json.load(f))
    objective = lambda x: rosenbrock(x, [True, False, False])["function"]
    plot_2d_history(objective, result["history"], result["xlims"], result["ylims"])

    plt.show()
