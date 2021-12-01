from time import sleep

with open("input", "r") as file:
    text = file.read().splitlines()

insts = list(enumerate(text))

acc = 0
ip = 0
jmps = []

while ip < len(insts):
    line_num, inst = insts[ip]
    if line_num in jmps:
        print('Loop detected!')
        break

    inst = inst.split()
    arg = inst[1]
    inst = inst[0]

    if inst == 'acc':
        acc += int(arg)
    
    if inst == 'jmp':
        ip += int(arg)
    else:
        ip += 1

    jmps.append(line_num)

print(f'Last acc value is {acc}')