from functools import reduce
import numpy as np
from scipy.ndimage import generic_filter

class ImageEnhanceer:
    def __init__(self, enhancement_algorithm) -> None:
        if len(enhancement_algorithm) != 512:
            raise ValueError("Algorithm has to have 512 elements")
        self.algorithm = enhancement_algorithm

    def enhance_filter(self, neighborhood):
        index = int(reduce(lambda acc, x: 2 * acc + x, neighborhood, 0))
        return self.algorithm[index]

    def enhance(self, image, steps=2):
        if self.algorithm[0] != 0 and steps % 2 != 0:
            raise ValueError("Can only perform an even amount of steps")
        if self.algorithm[0] != 0 and self.algorithm[-1] != 0 and steps > 0:
            raise ValueError("Image would have infinite light pixels")

        for i in range(steps):
            if i % 2 == 0 or self.algorithm[0] == 0:
                padding = 0
            else:
                padding = self.algorithm[0]
            image = np.pad(image, 1, constant_values=padding)
            image = generic_filter(image, self.enhance_filter, size=3, mode="constant", cval=padding)

        return image

def make_int(c):
    if c == '#':
        return 1
    elif c == '.':
        return 0
    else:
        raise ValueError(f"Cannot match char {c}")

def main():
    with open("./input", "r") as f:
        algorithm = np.array([make_int(c) for c in next(f).strip()])
        next(f) # skip blank line
        image = np.array([[make_int(c) for c in l.strip()] for l in f if l != ""])

    enhancer = ImageEnhanceer(algorithm)
    image = enhancer.enhance(image)
    print(image)
    print(image.sum())


if __name__ == "__main__":
    main()
