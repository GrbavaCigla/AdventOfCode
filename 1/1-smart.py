from itertools import combinations

with open("input", "r") as file:
    text = file.read().splitlines()

def two_point(values: list, target: int):
    start = 0
    end = len(values)-1

    while end > start:
        cur_sum = values[start] + values[end]

        if cur_sum == target:
            return values[start], values[end]

        elif cur_sum < target:
            start += 1
        
        else:
            end -= 1


values = sorted(map(int, text))

print(two_point(values, 2020))
