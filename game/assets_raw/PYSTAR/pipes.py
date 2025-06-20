import raylib as rl
from collections import deque
from typing import Optional


from enum import Enum
import random

class PipeType(Enum):
    SINGLE = 0
    LINE = 1
    CORNER = 2
    TEE = 3
    CROSS = 4

class Tile:
    def __init__(self, pipe_type, rotation):
        self.pipe_type = pipe_type
        self.rotation = rotation

ROWS, COLS = 10, 10

grid: list[list[Tile]] = []


def get_sides(pipe_type: PipeType, rotation: int):
    dirs = {
        PipeType.SINGLE: [0],
        PipeType.LINE: [0, 2],
        PipeType.CORNER: [0, 1],
        PipeType.TEE: [0, 1, 3],
        PipeType.CROSS: [0, 1, 2, 3],
    }
    return [(d + rotation) % 4 for d in dirs[pipe_type]]

def get_candidates(optional: list[int], include: list[int]) -> list[tuple[PipeType, int]]:
    candidates = []
    allowed_sides = set(optional) | set(include)
    for pipe_type in PipeType:
        for rotation in range(4):
            sides = set(get_sides(pipe_type, rotation))
            # Все обязательные стороны должны быть в sides
            if not set(include).issubset(sides):
                continue
            # sides должны быть подмножеством allowed_sides
            if not sides.issubset(allowed_sides):
                continue
            candidates.append((pipe_type, rotation))
    return candidates


PIPE_WEIGHTS = {
    PipeType.SINGLE: 10,
    PipeType.LINE: 10,
    PipeType.CORNER: 40,
    PipeType.TEE: 20,
    PipeType.CROSS: 10,
}

def pick_candidate(candidates: list[tuple[PipeType, int]]):
    weights = [PIPE_WEIGHTS[pipe_type] for pipe_type, _ in candidates]
    return random.choices(candidates, weights=weights, k=1)[0]


grid: list[list[Optional[Tile]]] = [[None for _ in range(COLS)] for _ in range(ROWS)]

def filter_grid_corners(pos: tuple[int, int], inc_sides: list[int]) -> list[int]:
    r, c = pos
    allowed = []
    for side in inc_sides:
        if side == 0 and r == 0:
            continue  # верхний край, нельзя вход сверху
        if side == 1 and c == COLS - 1:
            continue  # правый край, нельзя вход справа
        if side == 2 and r == ROWS - 1:
            continue  # нижний край, нельзя вход снизу
        if side == 3 and c == 0:
            continue  # левый край, нельзя вход слева
        allowed.append(side)
    return allowed

def neighbors(r, c):
    result = []
    if r > 0: result.append(((r-1, c), 0))   # сверху
    if c < COLS - 1: result.append(((r, c+1), 1))  # справа
    if r < ROWS - 1: result.append(((r+1, c), 2))  # снизу
    if c > 0: result.append(((r, c-1), 3))    # слева
    return result

def opposite_side(side):
    return (side + 2) % 4



def allowed_sides_grid_corners(pos: tuple[int, int], sides: list[int]) -> list[int]:
    r, c = pos
    allowed = sides.copy()
    if r == 0 and 0 in allowed: allowed.remove(0)   # верх недоступен
    if c == COLS - 1 and 1 in allowed: allowed.remove(1)  # право недоступно
    if r == ROWS - 1 and 2 in allowed: allowed.remove(2)  # низ недоступен
    if c == 0 and 3 in allowed: allowed.remove(3)   # лево недоступно
    return allowed

def get_include_exclude_sides(r: int, c: int):
    include = []
    exclude = []

    for (nr, nc), side_to_neighbor in neighbors(r, c):
        neighbor_tile = grid[nr][nc]
        if neighbor_tile is None:
            continue

        opposite = opposite_side(side_to_neighbor)
        neighbor_sides = get_sides(neighbor_tile.pipe_type, neighbor_tile.rotation)

        if opposite in neighbor_sides:
            # сосед смотрит на текущую клетку
            include.append(side_to_neighbor)
        else:
            # сосед не смотрит — исключаем эту сторону
            exclude.append(side_to_neighbor)

    return include, exclude


def generate_grid():
    global grid
    grid = [[None for _ in range(COLS)] for _ in range(ROWS)]

    start_r = random.randint(0, ROWS - 1)
    start_c = random.randint(0, COLS - 1)

    allowed = allowed_sides_grid_corners((start_r, start_c), [0,1,2,3])
    candidates = get_candidates(optional=allowed, include=[])
    grid[start_r][start_c] = Tile(*pick_candidate(candidates))

    queue = [(start_r, start_c)]
    while queue:
        r, c = queue.pop(0)
        for (nr, nc), side_to_neighbor in neighbors(r, c):
            if grid[nr][nc] is not None:
                continue

            include, exclude = get_include_exclude_sides(nr, nc)
            allowed = allowed_sides_grid_corners((nr, nc), [0,1,2,3])
            optional = [s for s in allowed if s not in include and s not in exclude]

            candidates = get_candidates(optional=optional, include=include)
            if not candidates:
                continue

                # raise Exception(f"No candidates for cell {(nr, nc)} include={include} exclude={exclude} optional={optional}")

            grid[nr][nc] = Tile(*pick_candidate(candidates))
            queue.append((nr, nc))

    return grid



generate_grid()

# First:  0 0 [(<PipeType.CORNER: 2>, 1)] <__main__.Tile object at 0x0000019E5795A4E0>
# Second:  0 1 [(<PipeType.SINGLE: 0>, 3), (<PipeType.CORNER: 2>, 3)] 


# ROWS, COLS = 3, 3
# inc_side = opposite_side(1)
# include = [inc_side]
# print(include) # [3] // ..right?
# exclude = exclude_grid_corners((0, 1), [0,1,2,3]) # 0 1 is pos
# print(exclude) # [1, 2, 3] // WRONG! Its ALLOWED!
# optional = [s for s in [0,1,2,3] if s not in include and s not in exclude]
# print(optional)
# candidates = get_candidates(optional, include)
# print(candidates) # [(<PipeType.SINGLE: 0>, 3), (<PipeType.CORNER: 2>, 3)] - must also include TEE, line and another corner
# grid[0][1] = Tile(*candidates[1]) # (<PipeType.CORNER: 2>, 3), but 3 rot

def draw_tile(x, y, size, tile: Tile):
    if tile is None: return
    cx = x + size // 2
    cy = y + size // 2
    r = size // 2 - 4
    angle = tile.rotation * 90

    # Helper: direction -> offset
    def dir_offset(dir_angle):
        a = (dir_angle + angle) % 360
        if a == 0: return (0, -r)
        if a == 90: return (r, 0)
        if a == 180: return (0, r)
        if a == 270: return (-r, 0)

    # Start
    rl.DrawCircle(cx, cy, 4, rl.BLACK)

    if tile.pipe_type == PipeType.SINGLE:
        dx, dy = dir_offset(0)
        rl.DrawLine(cx, cy, cx + dx, cy + dy, rl.DARKGRAY)

    elif tile.pipe_type == PipeType.LINE:
        for dir in [0, 180]:
            dx, dy = dir_offset(dir)
            rl.DrawLine(cx, cy, cx + dx, cy + dy, rl.DARKGRAY)

    elif tile.pipe_type == PipeType.CORNER:
        for dir in [0, 90]:
            dx, dy = dir_offset(dir)
            rl.DrawLine(cx, cy, cx + dx, cy + dy, rl.DARKGRAY)

    elif tile.pipe_type == PipeType.TEE:
        for dir in [0, 90, 270]:
            dx, dy = dir_offset(dir)
            rl.DrawLine(cx, cy, cx + dx, cy + dy, rl.DARKGRAY)

    elif tile.pipe_type == PipeType.CROSS:
        for dir in [0, 90, 180, 270]:
            dx, dy = dir_offset(dir)
            rl.DrawLine(cx, cy, cx + dx, cy + dy, rl.DARKGRAY)


def check_solved(grid):
    for r in range(ROWS):
        for c in range(COLS):
            tile = grid[r][c]
            if tile is None:
                return False
            sides = get_sides(tile.pipe_type, tile.rotation)
            for (nr, nc), side_to_neighbor in neighbors(r, c):
                if 0 <= nr < ROWS and 0 <= nc < COLS and grid[nr][nc] is not None:
                    neighbor = grid[nr][nc]
                    neighbor_sides = get_sides(neighbor.pipe_type, neighbor.rotation)
                    opposite = opposite_side(side_to_neighbor)
                    if (side_to_neighbor in sides) != (opposite in neighbor_sides):
                        return False
    return True



def main():
    rl.InitWindow(800, 600, b"Scalable Grid with Click")
    rl.SetTargetFPS(60)

    while not rl.WindowShouldClose():
        screen_width = rl.GetScreenWidth()
        screen_height = rl.GetScreenHeight()

        cell_width = screen_width / COLS
        cell_height = screen_height / ROWS

        rl.BeginDrawing()
        rl.ClearBackground(rl.RAYWHITE)

        cell_size = min(rl.GetScreenWidth() // COLS, rl.GetScreenHeight() // ROWS)

        for row in range(ROWS):
            for col in range(COLS):
                x = col * cell_size
                y = row * cell_size
                draw_tile(x, y, cell_size, grid[row][col])

        # Handle mouse click

        if rl.IsMouseButtonPressed(rl.MOUSE_LEFT_BUTTON):
            mx, my = rl.GetMousePosition().x, rl.GetMousePosition().y
            col = int(mx // cell_size)
            row = int(my // cell_size)
            if 0 <= row < ROWS and 0 <= col < COLS:
                grid[row][col].rotation = (grid[row][col].rotation + 1) % 4
            if check_solved(grid):
                print("Puzzle solved!")

        rl.EndDrawing()

    rl.CloseWindow()

if __name__ == "__main__":
    main()
