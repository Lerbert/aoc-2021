import torch
import torch.nn.functional as F

import numpy as np

def main():
    with open("./input", "r") as f:
            map = [[int(c) for c in l.strip()] for l in f if l != ""]
    
    map_t = torch.tensor(map)
    map_t = F.pad(map_t, (1, 1, 1, 1), value=11) # pad with 11s, 9 is highest input value, so this will never be the min
    map_t = map_t.unfold(0, 3, 1).unfold(1, 3, 1)
    map_t = map_t * torch.tensor([[10, 1, 10], [1, 1, 1], [10, 1, 10]]) # diagonal adjacencies don't count
    map_t = map_t - torch.tensor([[0, 1, 0], [1, 0, 1], [0, 1, 0]]) # center has to be strictly lower than adjacencies
    map_t = map_t.min(dim=-1)[0].min(dim=-1)[0]
    map_min = np.array(map_t)
    map_np = np.array(map)
    low_points = map_np[map_min == map_np]
    risk_score = low_points + 1
    print(risk_score.sum())
    

if __name__ == "__main__":
    main()
