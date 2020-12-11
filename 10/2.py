from pprint import pprint

with open("input", "r") as file:
    text = file.read().splitlines()
    text = sorted(map(int, text))


sol = {0:1}
for line in sorted(text):
    sol[line] = 0
    if line - 1 in sol:
        sol[line]+=sol[line-1]
    if line - 2 in sol:
        sol[line]+=sol[line-2]
    if line - 3 in sol:
        sol[line]+=sol[line-3]

print(sol[max(text)])
