import math

import torch
import numpy as np

with open("./input", "r") as f:
    for l in f:
        positions = [float(x) for x in l.strip().split(",")]
        break

input = torch.tensor(positions)
align_pos = torch.tensor(input.mean(), requires_grad=True)
loss_fn = torch.nn.L1Loss(reduction="sum")

lr = 0.1
optimizer = torch.optim.Adam((align_pos,), lr=lr)

for t in range(10000):
    loss = loss_fn(align_pos, input)
    optimizer.zero_grad()
    loss.backward()
    optimizer.step()
align = align_pos.item()
print(f"Aligning at position: {align} Fuel required: {loss.item()}")
print(f"Ceil: Aligning at position: {math.ceil(align)} Fuel required: {torch.abs(input - math.ceil(align)).sum()}")
print(f"Floor: Aligning at position: {math.floor(align)} Fuel required: {torch.abs(input - math.floor(align)).sum()}")