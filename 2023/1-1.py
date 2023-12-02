from string import digits

with open("input", "r") as file:
    inp = file.read().splitlines()

firsts = [[j for j in i if j in digits][0] for i in inp]
seconds = [[j for j in i if j in digits][-1] for i in inp]
print(sum(map(lambda x: int(x[0] + x[1]), zip(firsts, seconds))))