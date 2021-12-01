from typing import List

with open("1input", "r") as file:
    text = file.read().split('\n\n')

def get_side(tile, n):
    if n == 0:
        return ''.join([tile[i][0] for i in range(len(tile))])
    elif n == 1:
        return tile[0]
    elif n == 2:
        return ''.join([tile[i][-1] for i in range(len(tile))])
    elif n == 3:
        return tile[-1]

def match_sides(side1, side2):
    return side2 == side1 or side2[::-1] == side1

class Tile:
    tile: str
    taken_sides: List[bool]

    def __init__(self, tile, taken_side=[False] * 4):
        self.tile = tile
        self.taken_side = taken_side

tile1 = text[0].splitlines()
tile2 = text[1].splitlines()


for t1 in range(4):
    for t2 in range(4):
        print(match_sides(get_side(tile1, t1), get_side(tile2, t2)))
