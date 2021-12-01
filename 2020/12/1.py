with open("input", "r") as file:
    text = file.read().splitlines()

sides = {'E': 0, 'S': 0, 'W': 0, 'N': 0}
Ls = ['E', 'N', 'W', 'S']
Rs = ['E', 'S', 'W', 'N']

facing = 'E'

for line in text:
    di = line[0]
    amount = int(line[1:])

    if di in ['N', 'S', 'W', 'E']:
        sides[di] += amount
    
    if di == 'F':
        sides[facing] += amount

    if di == 'L':
        cur = Ls.index(facing)
        facing = int(cur + amount/90) % 4
        facing = Ls[facing] 

    if di == 'R':
        cur = Rs.index(facing)
        facing = int(cur + amount/90) % 4
        facing = Rs[facing]

print(abs(sides['E'] - sides['W']) + abs(sides['S'] - sides['N']))
