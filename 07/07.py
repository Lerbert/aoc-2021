import math

import torch
import numpy as np

def find_best_alignment(start, fuel_fn, lr=0.1, iter=10000):
    start_t = torch.tensor(start)
    align_pos = torch.tensor(np.mean(start), requires_grad=True)
    optimizer = torch.optim.Adam((align_pos,), lr=lr)

    for _ in range(iter):
        loss = fuel_fn(align_pos, start_t)
        optimizer.zero_grad()
        loss.backward()
        optimizer.step()

    align = align_pos.item()
    print(f"Float: Aligning at position: {align} Fuel required: {loss.item()}")
    ceil_align = math.ceil(align)
    ceil_fuel = fuel_fn(torch.tensor(ceil_align), start_t).item()
    print(f"Ceil:  Aligning at position: {ceil_align} Fuel required: {ceil_fuel}")
    floor_align = math.floor(align)
    floor_fuel = fuel_fn(torch.tensor(floor_align), start_t).item()
    print(f"Floor: Aligning at position: {floor_align} Fuel required: {floor_fuel}")
    int_align = ceil_align if ceil_fuel < floor_fuel else floor_align
    int_fuel = min(ceil_fuel, floor_fuel)
    print(f"Int:   Aligning at position: {int_align} Fuel required: {int_fuel}")
    return int_align, int_fuel

def main():
    with open("./input", "r") as f:
        for l in f:
            positions = [int(x) for x in l.strip().split(",")]
            break

    fuel_part1 = torch.nn.L1Loss(reduction="sum")
    def fuel_part2(y, t):
        diff = torch.abs(y - t)
        return torch.sum(diff * (diff + 1) / 2)
    
    print("=======Part 1=======")
    find_best_alignment(positions, fuel_part1)
    print("=======Part 2=======")
    find_best_alignment(positions, fuel_part2)
    

if __name__ == "__main__":
    main()
