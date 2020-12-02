with open("input", "r") as file:
    text = file.read().splitlines()

ans = 0
for line in text:
    limits, letter, password = line.split()

    lower, upper = limits.split("-")
    lower, upper = int(lower), int(upper)
    letter = letter[:-1]

    amount = password.count(letter)
    if lower <= amount <= upper:
        ans += 1

print(ans)