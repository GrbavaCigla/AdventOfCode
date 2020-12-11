from pprint import pprint

with open("input", "r") as file:
    text = file.read().splitlines()
    text = sorted(map(int, text))


def is_valid(seq, addend=False):
    if addend:
        seq = [0] + seq + [max(text) + 3]

    for i in range(1, len(seq)):
        if not (seq[i] - seq[i-1] in [3, 2, 1]):
            return False
    
    return True

res = []
def adapters(arr, n=0, sub=[]):
    if not is_valid(sub):
        return

    if n == len(arr):
        res.append(sub)
        return hash(tuple(sub))
    
    x = adapters(arr, n+1, sub)
    y = adapters(arr, n+1, sub+[arr[n]])

    return y, x

print("Starting")
adapters(text)
print("Done")

res = [i for i in res if is_valid(i, True)]

print(len(res))

# flatten = lambda lst: reduce(lambda l,i: l + flatten(i) if isinstance(i, tuple) else l + [i], lst, [])  

# result = flatten(adapters(text))
# # result = [[0] + i for i in result]
# result = list(set(map(tuple, result)))

# def is_valid(arr):
#     for i in range(1, len(arr)):
#         if not arr[i] - arr[i-1] in [1, 3]:
#             return False
#     return True



# result = [i for i in result if i[-1] == max(text)]
# result = [i for i in result if is_valid(i)]

# pprint(result)
