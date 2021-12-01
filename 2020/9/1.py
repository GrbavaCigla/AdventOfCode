import bisect

with open("input", "r") as file:
    text = file.read().splitlines()
    text = list(map(int, text))

def two_point(values: list, target: int):
    start = 0
    end = len(values)-1

    while end > start:
        cur_sum = values[start] + values[end]

        if cur_sum == target:
            return values[start], values[end]

        elif cur_sum < target:
            start += 1
        
        else:
            end -= 1

numbers = sorted(text[:25])

for i in text[25:]:
    bisect.insort(numbers, i)

    if not two_point(numbers, i):
        print(i)
        break
