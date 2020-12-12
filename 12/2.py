with open("input", "r") as file:
    text = file.read().splitlines()

ship = {'E': 0, 'S': 0, 'W': 0, 'N': 0}
waypoint = {'E': 10, 'S': 0, 'W': 0, 'N': 1}

Ls = ['E', 'N', 'W', 'S']
Rs = ['E', 'S', 'W', 'N']

facing = 'E'

def multiply_dict(pos, amount):
    pos = pos.copy()
    for k, v in pos.items():
        pos[k] = v * amount
    return pos

def manhattan(ship):
   return abs(ship['E'] - ship['W']) + abs(ship['S'] - ship['N'])

def add_dict(ship, add):
    for k, v in ship.items():
        ship[k] = v + add[k]
    return ship

def shift_dict(ship, amount):
    keys = list(ship.keys())
    vals = list(ship.values())

    for i, key in enumerate(keys):
        ship[key] = vals[(i+amount)%len(vals)]
    
    return ship


for line in text:
    di = line[0]
    amount = int(line[1:])

    if di in ['N', 'S', 'W', 'E']:
        waypoint[di] += amount
    
    if di == 'F':
        ship = add_dict(multiply_dict(waypoint, amount), ship)

    if di == 'L':
        shift_dict(waypoint, int(amount/90))

    if di == 'R':
        shift_dict(waypoint, int(-amount/90))
        
print(manhattan(ship))
