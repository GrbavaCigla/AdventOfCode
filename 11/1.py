from pprint import pprint

with open("input", "r") as file:
    text = file.read().replace("L", "#").splitlines()

def get_surround(text, x, y):
    surround = ""

    xlower = -1 if x > 0 else 0
    ylower = -1 if y > 0 else 0

    for i in range(xlower, 2):
        for j in range(ylower, 2):
            if i == 0 and j == 0:
                continue
            
            try:
                surround += text[y+j][x+i]
            except:
                pass

    return surround
while True:
    seats = text.copy()

    for y in range(len(text)):

        ystr = ""
        for x in range(len(text[y])):

            if text[y][x] == '.':
                ystr += '.'
                continue

            surround = get_surround(text, x, y)

            if surround.count("#") == 0:
                ystr += "#"
                continue

            if surround.count("#") >= 4:
                ystr += "L"
                continue

            ystr += text[y][x]

        # print(ystr)
        seats[y] = ystr
    
    print(sum([i.count('#') for i in seats]))

    del text
    text = seats.copy()

