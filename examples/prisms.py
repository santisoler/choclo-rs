import time
import numba
import numpy as np
import verde as vd
import matplotlib.pyplot as plt

import choclo
from choclors import points_gz


@numba.jit(nopython=True, parallel=True)
def points_gz_choclo(easting, northing, upward, points, masses):
    n_coords = easting.size
    n_points = masses.size
    result = np.ones_like(easting)
    for i in numba.prange(n_coords):
        for j in range(n_points):
            result[i] += -choclo.point.gravity_u(
                easting[i],
                northing[i],
                upward[i],
                points[0][j],
                points[1][j],
                points[2][j],
                masses[j],
            )
    return result


region = (-100.0, 100.0, -100.0, 100.0)

grid_coords = vd.grid_coordinates(region, spacing=5, extra_coords=0)
easting, northing, upward = tuple(c.ravel() for c in grid_coords)
# points = np.array([[0], [0], [-20]], dtype=np.float64)
# masses = np.array([1e6], dtype=np.float64)

n_points = 200
points = np.array(vd.scatter_points(region, size=n_points, extra_coords=-10))
masses = 1e6 * np.ones(n_points)

start = time.time()
result_rust = points_gz(easting, northing, upward, points, masses)
result_rust = result_rust.reshape(grid_coords[0].shape)
end = time.time()
print(f"Elapsed time (Rust): {end - start}s")

# Run first time to compile
result_numba = points_gz_choclo(easting, northing, upward, points, masses)
start = time.time()
result_numba = points_gz_choclo(easting, northing, upward, points, masses)
result_numba = result_rust.reshape(grid_coords[0].shape)
end = time.time()
print(f"Elapsed time (Numba): {end - start}s")

fig, (ax1, ax2) = plt.subplots(ncols=2, sharey=True)
tmp = ax1.pcolormesh(*grid_coords[:2], result_rust)
plt.colorbar(tmp, ax=ax1)

tmp = ax2.pcolormesh(*grid_coords[:2], result_numba)
plt.colorbar(tmp, ax=ax2)

for ax in (ax1, ax2):
    ax.set_aspect("equal")
plt.show()
