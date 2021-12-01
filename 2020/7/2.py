import re
from pprint import pprint

def in_bag(bags, bag):
    for i in bags:
        if bag in i:
            return True

    return False

total = 0
def grepit(bags, target, bamount=1):
    bag = re.findall(r"[a-z]+ [a-z]+", target)[0]
    amount = int(re.findall(r"\d+", target)[0])*bamount
    inner = bags[bag]
    
    if not inner:
        return amount

    global total
    total += amount
    return [grepit(bags, i, amount) for i in inner]

def flatten(list_of_lists):
    if len(list_of_lists) == 0:
        return list_of_lists
    if isinstance(list_of_lists[0], list):
        return flatten(list_of_lists[0]) + flatten(list_of_lists[1:])
    return list_of_lists[:1] + flatten(list_of_lists[1:])

with open("input", "r") as file:
    text = file.read().splitlines()
    master_bags = [re.findall(r"^\w+ \w+", i)[0] for i in text]
    secondary_bags = [re.findall(r"\d+ \w+ \w+", i) for i in text]

    bags = dict(zip(master_bags, secondary_bags))
    print(sum(flatten(grepit(bags, '1 shiny gold'))) + total - 1)