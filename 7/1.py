import re

def in_bag(bags, bag):
    for i in bags:
        if bag in i:
            return True

    return False

def grepit(bags, targets):
    todo = []

    for target in targets:
        for k, v in bags.items():
            if in_bag(v, target):
                todo.append(k)

    if not todo:
        return []

    return todo + grepit(bags, todo)

with open("input", "r") as file:
    text = file.read().splitlines()
    master_bags = [re.findall(r"^\w+ \w+", i)[0] for i in text]
    secondary_bags = [re.findall(r"\d+ \w+ \w+", i) for i in text]

    bags = dict(zip(master_bags, secondary_bags))
    print(
        len(set(grepit(bags, ['shiny gold'])))
    )