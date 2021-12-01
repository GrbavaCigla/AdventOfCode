with open("input", "r") as file:
    text = file.read().splitlines()

ans = 0
for line in text:
    limits, letter, password = line.split()

    lower, upper = limits.split("-")
    lower, upper = int(lower)-1, int(upper)-1
    letter = letter[:-1]

    if (password[lower] == letter) != (password[upper] == letter):
        ans += 1

print(ans)