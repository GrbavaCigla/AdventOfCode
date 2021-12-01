from tqdm import tqdm

nums = [1,20,11,6,12,0]

spoken = nums.copy()
spoken.pop(-1)

# def recent_two(text, target):
#     rev_text = list(reversed(text))
#     most_recent = len(text) - rev_text.index(target)
#     recent = len(text) - rev_text.index(len(text) - most_recent)

#     return most_recent - recent

def most_recent(nums, target):
    rev_nums = nums[::-1]
    most_recent = len(nums) - rev_nums.index(target)

    return most_recent

for i in tqdm(range(len(nums) - 1, 2020)):
    if nums[i] in spoken:
        ms = i + 1 - most_recent(nums[:-1], nums[i])
        nums.append(ms)
        # print(f'SPOKEN: n:{nums}, s:{spoken}, c:{nums[i]}, i:{i}, f:{ms}')
    else:
        nums.append(0)
        # print(f'NOSPOKEN: n:{nums}, s:{spoken}, c:{nums[i]}, i:{i}')

    spoken.append(nums[i])

print(nums[-2])