with open("input", "r") as file:
    text = file.read().splitlines()
    text = sorted(map(int, text))

jolt3 = 1
jolt1 = 1

for i in range(1, len(text)):
    jolt3 += 1 if text[i] - text[i-1] == 3 else 0
    jolt1 += 1 if text[i] - text[i-1] == 1 else 0

print(jolt3*jolt1)