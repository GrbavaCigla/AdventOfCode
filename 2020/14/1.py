import re

with open('input', 'r') as file:
    input = file.read().splitlines()

def mask_num(mask, num):
    ans = ''

    padded = str(num).rjust(36, '0')

    for i in range(len(mask)):
        if 'X' == mask[i]:
            ans += padded[i]
        else:
            ans += mask[i]

    print(f'VALUE: {padded}\nMASK:  {mask}\nRESULT:{ans}\n')
    
    return int(ans, 2)

mask = ""
mem = dict()

for line in input:
    if line.startswith("mask"):
       mask = line.split(' = ')[1]
    
    else:
        splited = line.split(' = ')

        address = int(re.findall(r'\d+', splited[0])[0])
        num = int(splited[1])
        mem[address] = mask_num(mask, bin(num)[2:])



    print(f'MASK {mask} - MEM: {mem}')

print(sum(mem.values()))
