import numpy as np
from scipy.ndimage import generic_filter

def transfer_energy(neighborhood):
    neighborhood = neighborhood.reshape((3, 3))
    center = neighborhood[1, 1]
    energy_increase = np.sum(neighborhood == 10)
    # Mark octopuses that already flashed with 11, so they won't transfer or receive energy again
    # Clamp energy to 10 so that we can detect freshly flashed octopuses
    return min(center + energy_increase, 10) if center < 10 else 11

def octopus_step(octopuses):
    octopuses = octopuses + 1
    while np.any(octopuses == 10):
        octopuses = generic_filter(octopuses, transfer_energy, size=3, mode="constant")
    flashes = np.sum(octopuses > 9)
    octopuses[octopuses > 9] = 0
    return octopuses, flashes

def simulate_octopuses(octopuses, steps):
    total_flashes = 0
    for _ in range(steps):
        octopuses, flashes = octopus_step(octopuses)
        total_flashes += flashes
    return octopuses, total_flashes

def find_first_all_flash(octopuses):
    step = 0
    while True:
        step += 1
        octopuses, flashes = octopus_step(octopuses)
        if flashes == np.ma.size(octopuses):
            break
    return step

def main():
    with open("./input", "r") as f:
        octopuses = np.array([[int(c) for c in l.strip()] for l in f if l != ""])

    steps = 100
    _, flashes = simulate_octopuses(octopuses, steps)
    print(f"After {steps} steps there have been {flashes} flashes in total")

    first_all_flash = find_first_all_flash(octopuses)
    print(f"After {first_all_flash} steps all octopuses flash together for the first time")


if __name__ == "__main__":
    main()
