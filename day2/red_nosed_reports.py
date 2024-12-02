import itertools as it

test_input = """
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"""

def parse(input: str):
    lines = input.splitlines()
    output = []
    for line in lines:
        if line == "":
            continue
        output.append([int(n) for n in line.split(' ') if n != ''])
    return output

def get_steps(levels: list[int]):
    return [b - a for (a, b) in it.pairwise(levels)]

def steps_are_safe(steps: list[int]):
    all_pos = all(x > 0 for x in steps)
    all_neg = all(x < 0 for x in steps)
    within_range = all(1 <= abs(x) <= 3 for x in steps)
    return (all_pos or all_neg) and within_range

def num_safe_levels(all_levels: list[list[int]]):
    return len([levels for levels in all_levels if steps_are_safe(get_steps(levels))])

def steps_are_safe2(levels: list[int]):
    if steps_are_safe(get_steps(levels)):
        return True
    for i in range(len(levels)):
        if steps_are_safe(get_steps(levels[:i] + levels[i+1:])):
            return True
    return False

def num_safe_levels2(all_levels: list[list[int]]):
    return len([levels for levels in all_levels if steps_are_safe2(levels)])

with open("input.txt", 'r') as file:
    contents = file.read()
    print("Part 1:", num_safe_levels(parse(contents)))
    print("Part 2:", num_safe_levels2(parse(contents)))
