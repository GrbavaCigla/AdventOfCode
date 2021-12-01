with open("input", "r") as file:
    text = file.read().splitlines()
    timestamp = int(text[0])
    
    buses = text[1].split(',')
    nums = [int(i) for i in buses if not i.isalpha()]

curtime = 0

flag = False

while not flag:
    curtime += int(buses[0])

    nextpos = 0
    good = 0
    for i in buses:
        if not i.isalpha():
            i = int(i)
            if (curtime+nextpos)%i == 0:
                good += 1
        
        nextpos += 1

    if good == len(nums):
        print(nextpos+curtime-len(buses))
        break
