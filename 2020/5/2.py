with open("input", "r") as file:
    text = file.read().splitlines()

def decode_seat(seat: str):
    lower = 0
    upper = 127
    left = 0
    right = 7

    ans1, ans2 = 0, 0

    for i, letter in enumerate(seat):
        if letter == 'F':
            upper = lower + (upper - lower)//2
            ans1 = upper
        elif letter == 'B':
            lower = lower + (upper - lower)//2 + 1
            ans1 = lower
        
        elif letter == 'L':
            right = left + (right - left)//2
            ans2 = right
        elif letter == 'R':
            left = left + (right - left)//2 + 1
            ans2 = left
        
    return ans1*8+ans2

ids = sorted([decode_seat(i) for i in text])
for i, j in enumerate(ids[1:]):
    if j - ids[i] == 2:
        print((j + ids[i])//2)
