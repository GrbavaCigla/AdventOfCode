from time import sleep

with open("input", "r") as file:
    text = file.read().splitlines()


insts = list(enumerate(text))
change_line = 0

for i in range(len(insts)):
    new_line_num, inst = insts[i]
    inst = inst.split()
    new_arg = inst[1]
    inst = inst[0]
    if inst == 'jmp':
        insts[new_line_num] = (new_line_num, f'nop {new_arg}')
        change_line = new_line_num
    else:
        continue

    acc = 0
    ip = 0
    jmps = []
    loop = False

    while ip < len(insts):
        line_num, inst = insts[ip]
        if line_num in jmps:
            loop = True
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
        
        # print(ip, acc, inst, arg, jmps)
    
    insts[new_line_num] = (new_line_num, f'jmp {new_arg}')
    if not loop:
        print(acc)
        break
    
