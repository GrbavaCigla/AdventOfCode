import re

with open("input", "r") as file:
    text = file.read().split('\n\n')
    text = [i.replace('\n', ' ') for i in text]
    text = [i.replace(' ', ':') for i in text]
    text = [i.split(':')[::2] for i in text]

ans = 0
for passport in text:
    if len(passport) == 8 or (len(passport) == 7 and 'cid' not in passport):
        ans+=1

print(ans)