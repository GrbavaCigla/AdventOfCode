from string import digits

with open("input", "r") as file:
    inp = file.read().splitlines()

names = [
    "one", 
    "two", 
    "three", 
    "four", 
    "five", 
    "six", 
    "seven", 
    "eight", 
    "nine",
] + [i for i in digits]

bla = []

for _ in range(2):
    inds = [min([(i.find(j), j) for j in names if i.find(j) > -1]) for i in inp]
    inds = [names.index(i[1]) % 10 + 1 for i in inds]
    bla.append(inds)

    names = [i[::-1] for i in names]
    inp = [i[::-1] for i in inp]

print(sum(map(lambda x: int(str(x[0]) + str(x[1])), zip(bla[0], bla[1]))))
