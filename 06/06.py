import numpy as np

def simulate_fish_growth(trans_matrix, initial_fishes, day):
    fish_count = np.sum(np.linalg.matrix_power(trans_matrix, day).dot(initial_fishes))
    print(f"There are {fish_count} fishes on day {day}")

with open("./input", "r") as f:
    for l in f:
        numbers = [int(x) for x in l.strip().split(",")]
        break
fishes = np.array([len(list(filter(lambda x: x == i, numbers))) for i in range(9)])

transition_matrix = np.diag(np.ones(8), 1) # Advance fishes by one position
transition_matrix[6, 0] = 1 # Fishes reset timer to 6
transition_matrix[-1, 0] = 1 # Fishes with timer 0 spawn a new fish with timer 8

simulate_fish_growth(transition_matrix, fishes, 80)
simulate_fish_growth(transition_matrix, fishes, 256)
