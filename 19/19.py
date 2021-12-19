import numpy as np
from scipy import spatial

class Scanner:
    def __init__(self, beacons) -> None:
        self.beacons = beacons
        self.calculate_distance_matrix()

    def add_beacons(self, other):
        """If possible, add the beacons detected by the other scanner to our beacons
        
        For this try to find the transformation from other's coordinate system into ours.
        If this is not possible, return false and don't add anything to our beacons.
        """
        transformation = other.calculate_transformation(self)
        if transformation is not None:
            rotation, translation = transformation
            transformed_beacons = other.beacons.dot(rotation.T) + translation
            self.beacons = np.unique(np.concatenate((self.beacons, transformed_beacons)), axis=0)
            self.calculate_distance_matrix()
            return True
        return False

    def calculate_distance_matrix(self):
        self.beacon_distances = spatial.distance_matrix(self.beacons, self.beacons)

    def calculate_transformation(self, other):
        """Rotation matrix and translation vector to transform self's coordinates into those of other
        """
        point_map = self.overlapping_beacons(other)
        if point_map is not None:
            my_matching_beacons = self.beacons[list(sorted(point_map.values()))]
            other_matching_beacons = list(map(lambda x: x[1], sorted(filter(lambda e: e[0] in point_map, enumerate(other.beacons)), key=lambda e: point_map[e[0]])))
            system = equation_system(my_matching_beacons, other_matching_beacons)
            self.rotation, self.translation = get_rotation_and_translation(system[0], system[1])
            return self.rotation, self.translation
        return None

    def overlapping_beacons(self, other):
        for my_dist in self.beacon_distances:
            for other_dist in other.beacon_distances:
                my_matching = np.in1d(my_dist, other_dist)
                other_matching = np.in1d(other_dist, my_dist)
                if my_matching.sum() >= 12 and other_matching.sum() >= 12:
                    point_map = {} # Maps indices in other's beacons to indices for self's
                    for i, d in zip(np.argwhere(other_matching).flatten(), other_dist[other_matching]):
                        a = np.argwhere(my_dist == d).flatten()
                        assert(len(a) == 1)
                        point_map[i] = a[0]
                    return point_map
        return None

# The scanners perform a basis transformation with Ax + b where A represents the rotation (only 90°, so every entry is 0 or 1) and b represents the translation
# Thus, we have 12 variables we have to solve for: v = [a11, a12, a13, a21, ..., a33, b1, b2, b3]^T
# For a given pair of matching points x and y this means solving
# [x1 x2 x3 0  0  0  0  0  0  1  1  1]           [y1]
# |0  0  0  x1 x2 x3 0  0  0  1  1  1|   *   v = |y2|
# [0  0  0  0  0  0  x1 x2 x3 1  1  1]           [y3]

def coefficient_matrix(x):
    """Get the coefficient of the linear system of equations to solve for two matching points x and y.
    """
    zeros = np.zeros_like(x)
    part1 = np.concatenate((x, zeros, zeros, [1, 0, 0]))
    part2 = np.concatenate((zeros, x, zeros, [0, 1, 0]))
    part3 = np.concatenate((zeros, zeros, x, [0, 0, 1]))
    a = np.vstack((part1, part2, part3))
    return a

def equation_system(points_x, points_y):
    """Equation system to calculate rotation and translation neccessary to transform points_x into points_y
    """
    a = [coefficient_matrix(p) for p in points_x]
    b = np.concatenate(points_y)
    return np.vstack(tuple(a)), b

def get_rotation_and_translation(a, b):
    s = np.linalg.lstsq(a, b, rcond=None)[0]
    s = np.round(s)
    rotation = s[:9].reshape((3, 3))
    translation = s[9:]
    return rotation, translation

with open("./input", "r") as f:
    scanners = []
    beacons = []
    for l in f:
        l = l.strip()
        if l == "":
            scanners.append(Scanner(np.array(beacons)))
            beacons = []
        elif l.startswith("--- scanner "):
            pass
        else:
            beacons.append(np.array(l.split(","), dtype=float))
if beacons:
    scanners.append(Scanner(np.array(beacons)))
scanners = np.array(scanners)

integrated_scanners = np.zeros_like(scanners, dtype=bool)
s_origin = scanners[0]
integrated_scanners[0] = True
while not integrated_scanners.all():
    for i, scanner in enumerate(scanners):
        if not integrated_scanners[i]:
            integrated_scanners[i] = s_origin.add_beacons(scanner)

print(s_origin.beacons)
print(len(s_origin.beacons))
