"""
A very very ugly python solve for puzzle 8,
But I was to tired to try Rust on this one or to think about readability/optimization.
"""
with open("input/input_8.txt") as f:
    grid = [[int(i) for i in str.strip(s)] for s in f.readlines()]
    count = 0

    for i in range(len(grid)):
        for j in range(len(grid[0])):
            tree = grid[i][j]
            if i == 0 or i == len(grid) - 1 or j == 0 or j == len(grid[0]) - 1:
                count += 1
                continue

            for y in range(0, i):
                if grid[y][j] >= tree:
                    break
            else:
                count += 1
                continue

            for y in range(i + 1, len(grid)):
                if grid[y][j] >= tree:
                    break
            else:
                count += 1
                continue

            for x in range(0, j):
                if grid[i][x] >= tree:
                    break
            else:
                count += 1
                continue

            for x in range(j + 1, len(grid[0])):
                if grid[i][x] >= tree:
                    break
            else:
                count += 1
                continue

    print("Part 1: " + str(count))


    best = 0
    for i in range(len(grid)):
        for j in range(len(grid[0])):
            tree = grid[i][j]
            
            if i == 0 or i == len(grid) - 1 or j == 0 or j == len(grid[0]) - 1:
                continue
            score = 1

            # Top
            counter = 0
            for y in reversed(range(0, i)):
                counter += 1
                if grid[y][j] >= tree:
                    break
            score *= counter

            # Bottom
            counter = 0
            for y in range(i + 1, len(grid)):
                counter += 1
                if grid[y][j] >= tree:
                    break
            score *= counter

            # Left
            counter = 0
            for x in reversed(range(0, j)):
                counter += 1
                if grid[i][x] >= tree:
                    break
            score *= counter

            # Right
            counter = 0
            for x in range(j + 1, len(grid[0])):
                counter += 1
                if grid[i][x] >= tree:
                    break
            score *= counter

            if score > best:
                best = score

    print(f"Part 2: {best}")
