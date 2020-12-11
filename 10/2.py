with open("input", "r") as file:
    text = file.read().splitlines()
    text = sorted(map(int, text))


seqs = {0:1}
for line in text:
    seqs[line] = 0
    if line - 1 in seqs:
        seqs[line] += seqs[line-1]
    if line - 2 in seqs:
        seqs[line] += seqs[line-2]
    if line - 3 in seqs:
        seqs[line] += seqs[line-3]

print(seqs[max(text)])
