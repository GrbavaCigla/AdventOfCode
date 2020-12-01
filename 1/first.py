from itertools import combinations

with open("input", "r") as file:
    text = file.read().splitlines()

text = list(map(int, text))

for i in combinations(text, 2):
    if i[0] + i[1] == 2020:
        print(i)
