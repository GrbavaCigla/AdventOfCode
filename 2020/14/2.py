import re
from itertools import product

with open('input', 'r') as file:
    input = file.read().splitlines()

def get_vars(address, amount):
    new_address = address

    for i in product(*[(0, 1)] * amount):
        for c in i:
            new_address = new_address.replace('X', str(c), 1)
       
        yield new_address
        new_address = address


def mask_address(mask, address):
    address = bin(address)[2:].rjust(36, '0')
    
    new_address = ''

    for i in range(len(mask)):
        if mask[i] == '0':
            new_address += address[i]
        else:
            new_address += mask[i]


    return list(get_vars(new_address, mask.count('X')))
        

mask = ""
mem = dict()

for line in input:
    if line.startswith("mask"):
       mask = line.split(' = ')[1]
    
    else:
        splited = line.split(' = ')

        address = int(re.findall(r'\d+', splited[0])[0])
        num = int(splited[1])
        
        mask_address(mask, address)
        for i in mask_address(mask, address):
            mem[i] = num


    # print(f'MASK {mask} - MEM: {mem}')

print(sum(mem.values()))
