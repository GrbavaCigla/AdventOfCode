from itertools import combinations

with open("input", "r") as file:
    text = file.read().splitlines()

text = list(map(int, text))

for i in combinations(text, 3):
    if i[0] + i[1] + i[2] == 2020:
        print(i)
