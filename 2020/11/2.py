with open("input", "r") as file:
    text = file.read().splitlines()

def get_surround(text, x, y):
    surround = ""

    xlower = -1 if x > 0 else 0
    ylower = -1 if y > 0 else 0

    for i in range(xlower, 2):
        for j in range(ylower, 2):
            if i == 0 and j == 0:
                continue
            
            cur = '.'
            xcount = x
            ycount = y

            while cur == '.':
                xcount += i
                ycount += j

                if xcount < 0 or ycount < 0:
                    break
                
                if xcount == x and ycount == y:
                    continue

                try:
                    cur = text[ycount][xcount]
                except:
                    break
                
            surround += cur

            
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

            if surround.count("#") >= 5:
                ystr += "L"
                continue

            ystr += text[y][x]

        seats[y] = ystr
    
    print(sum([i.count('#') for i in seats]))

    del text
    text = seats.copy()
