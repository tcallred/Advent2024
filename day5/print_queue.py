test_input = """
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"""

def parse_input(input: str):
    sections = input.strip().split("\n\n")
    top = sections[0]
    bottom = sections[1]
    rules = [(int(line.split('|')[0]), int(line.split('|')[1])) for line in top.splitlines()]
    pagelists = [[int(x) for x in line.split(",")] for line in bottom.splitlines()]
    return (rules, pagelists)

def make_rules_map(rulelist):
    rules = {}
    for (a, b) in rulelist:
        if a in rules:
            rules[a].append(b)
        else:
            rules[a] = [b]
    return rules

def part1(rulelist, pagelists):
    rules = make_rules_map(rulelist)
    total = 0
    for pagelist in pagelists:
        valid = True
        for (i, page) in enumerate(pagelist):
            if not page in rules:
                continue
            for p in pagelist[:i]:
                if p in rules[page]:
                    valid = False
        if valid:
            total += pagelist[len(pagelist)//2]
    return total

def part2(rulelist, pagelists):
    rules = make_rules_map(rulelist)
    total = 0
    for pagelist in pagelists:
        swapped = False
        for (i, page) in enumerate(pagelist):
            if not page in rules:
                continue
            for (j, p) in enumerate(pagelist[:i]):
                if p in rules[page]:
                    pagelist[i], pagelist[j] = pagelist[j], pagelist[i]
                    swapped = True
        if swapped:
            total += pagelist[len(pagelist)//2]
    return total


with open("input.txt", 'r') as file:
    contents = file.read()
    (rulelist, pagelists) = parse_input(contents)
    print(part1(rulelist, pagelists))
    print(part2(rulelist, pagelists))
