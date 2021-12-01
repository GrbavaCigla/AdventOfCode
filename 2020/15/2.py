from tqdm import tqdm

nums = [1,20,11,6,12,0]

memo = {e: i for i, e in enumerate(nums[:-1])}

for i in tqdm(range(len(nums) - 1, 30000000)):
    if memo.get(nums[i]) != None:
        ms = i - memo[nums[i]]
        nums.append(ms)
        # print(f'SPOKEN: n:{nums}, m:{memo}, c:{nums[i]}, i:{i}, f:{ms}')
    else:
        nums.append(0)
        # print(f'NOSPOKEN: n:{nums}, m:{memo}, c:{nums[i]}, i:{i}')

    memo[nums[i]] = i

print(nums[-2])