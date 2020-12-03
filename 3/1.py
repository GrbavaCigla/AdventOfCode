with open("input", "r") as file:
    text = file.read().splitlines()

limit = len(text[0])
xcount = 0
ans = 0

for line in text[1:]:
    xcount+=3
    current = line[xcount % limit]

    ans += 1 if current == "#" else 0

print(ans)