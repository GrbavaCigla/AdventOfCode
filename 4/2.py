import re

with open("input", "r") as file:
    text = file.read().split('\n\n')
    text = [i.replace('\n', ' ') for i in text]
    text = [i.replace(' ', ':') for i in text]

    keys = [i.split(':')[::2] for i in text]
    values = [i.split(':')[1::2] for i in text]


ans = 0
for fields, values in zip(keys, values):
    if len(fields) != 8 and (len(fields) != 7 or 'cid' in fields):
        continue
    
    curdict = dict(zip(fields, values))

    byr = int(curdict['byr'])
    if byr < 1920 or byr > 2002:
        continue

    iyr = int(curdict['iyr'])
    if iyr < 2010 or iyr > 2020:
        continue

    eyr = int(curdict['eyr'])
    if eyr < 2020 or eyr > 2030:
        continue
    
    hgt = curdict['hgt']
    if 'in' in hgt:
        hgt = int(hgt[:-2])
        if hgt < 59 or hgt > 76:
            continue
    elif 'cm' in hgt:
        hgt = int(hgt[:-2])
        if hgt < 150 or hgt > 193:
            continue
    else:
        continue

    hcl = curdict['hcl']
    if not re.match(r"^#([a-fA-F0-9]{6})$", hcl):
        continue

    ecl = curdict['ecl']
    if ecl not in ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth']:
        continue

    pid = curdict['pid']
    if not re.match(r"^\d{9}$", pid):
        continue

    ans+=1

print(ans)