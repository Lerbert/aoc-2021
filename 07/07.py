import math

import torch
import numpy as np

with open("./input", "r") as f:
    for l in f:
        positions = [int(x) for x in l.strip().split(",")]
        break

input = torch.tensor(positions)
align_pos = torch.tensor(np.mean(positions), requires_grad=True)
loss_part1 = torch.nn.L1Loss(reduction="sum")
def loss_part2(y, t):
    diff = torch.abs(y - t)
    return torch.sum(diff * (diff + 1) / 2)

lr = 0.1
optimizer = torch.optim.Adam((align_pos,), lr=lr)

for t in range(10000):
    loss = loss_part2(align_pos, input)
    optimizer.zero_grad()
    loss.backward()
    optimizer.step()
align = align_pos.item()
print(f"Aligning at position: {align} Fuel required: {loss.item()}")
print(f"Ceil:  Aligning at position: {math.ceil(align)} Fuel required: {loss_part2(math.ceil(align), input).item()}")
print(f"Floor: Aligning at position: {math.floor(align)} Fuel required: {loss_part2(math.floor(align), input).item()}")
