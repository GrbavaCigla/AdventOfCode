with open("input", "r") as fp:
    lines = fp.readlines()

LINES=lines
start = int(LINES[0])
busses = ["x" if x == "x" else int(x) for x in LINES[1].split(",")]

def part2():
    mods = {bus: -i % bus for i, bus in enumerate(busses) if bus != "x"}
    print(mods)
    vals = list(reversed(sorted(mods)))
    val = mods[vals[0]]
    r = vals[0]
    for b in vals[1:]:
        while val % b != mods[b]:
            val += r
        r *= b
    return val

print(part2())

