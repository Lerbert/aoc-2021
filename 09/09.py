import torch
import torch.nn.functional as F

import numpy as np


def get_minimum_map(map):
    map_t = torch.tensor(map)
    map_t = F.pad(
        map_t, (1, 1, 1, 1), value=11
    )  # pad with 11s, 9 is highest input value, so this will never be the min
    map_t = map_t.unfold(0, 3, 1).unfold(
        1, 3, 1
    )  # expand each point to its direct neighborhood
    map_t = map_t * torch.tensor(
        [[10, 1, 10], [1, 1, 1], [10, 1, 10]]
    )  # diagonal adjacencies don't count
    map_t = map_t - torch.tensor(
        [[0, 1, 0], [1, 0, 1], [0, 1, 0]]
    )  # center has to be strictly lower than adjacencies
    return np.array(
        map_t.min(dim=-1)[0].min(dim=-1)[0]
    )  # map each neighborhood to its minimum


def find_low_points(map):
    minimum_map = get_minimum_map(map)
    return map[
        map == minimum_map
    ]  # low points are the points that equal the minimum in their modified neighborhood


def find_basin_sizes(map):
    basin_map = np.zeros_like(map)
    basin_map[map == 9] = -1
    minimum_map = get_minimum_map(map)
    basin_map[map == minimum_map] = 1
    basin_map = np.hstack(
        (
            -np.ones_like(basin_map[:, 0]).reshape((-1, 1)),
            basin_map,
            -np.ones_like(basin_map[:, 0]).reshape((-1, 1)),
        )
    )
    basin_map = np.vstack(
        (
            -np.ones_like(basin_map[0, :]).reshape((1, -1)),
            basin_map,
            -np.ones_like(basin_map[0, :]).reshape((1, -1)),
        )
    )

    # now spread numbers to neighbors that are lower, but non-negative
    changed = True
    neighbor_idx = [(1, 0), (-1, 0), (0, 1), (0, -1)]
    idx_to_update = [
        [[(i, j)] for j in range(len(basin_map[0]))] for i in range(len(basin_map))
    ]
    while changed:
        changed = False
        for i, l in enumerate(basin_map):
            for j, center in enumerate(l):
                if center <= 0:
                    continue
                growth = 0
                for idx_incr in neighbor_idx:
                    # since we padded with -1, all those should exist
                    idx = i + idx_incr[0], j + idx_incr[1]
                    if basin_map[idx] == 0:
                        changed = True
                        growth += 1
                        idx_to_update[idx[0]][idx[1]] = idx_to_update[i][j]
                        idx_to_update[i][j] += [idx]
                        basin_map[idx] = center
                for idx in idx_to_update[i][j]:
                    basin_map[idx] += growth

    basin_map = basin_map[1:-1, 1:-1]  # cut padding
    return basin_map[map == minimum_map]  # basin size at low points


def main():
    with open("./input", "r") as f:
        map = np.array([[int(c) for c in l.strip()] for l in f if l != ""])

    low_points = find_low_points(map)
    risk_score = low_points + 1
    print(f"The sum of all risk levels is {risk_score.sum()}")

    basin_sizes = find_basin_sizes(map)
    print(
        f"The product of the three largest basins is {np.prod(sorted(basin_sizes, reverse=True)[:3])}"
    )


if __name__ == "__main__":
    main()
