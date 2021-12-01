from functools import reduce
import operator

with open("input", "r") as file:
    text = file.read().splitlines()

limit = len(text[0])

steps = [
    (1, 1),
    (3, 1),
    (5, 1),
    (7, 1),
    (1, 2)
]

anss = []
for x, y in steps:
    ans = 0
    xcount = 0

    for line in text[y::y]:
        xcount += x
        current = line[xcount % limit]
        
        ans += 1 if current == "#" else 0

    anss.append(ans)

print(reduce(operator.mul, anss, 1))
