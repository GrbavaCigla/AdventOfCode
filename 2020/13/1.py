with open("input", "r") as file:
    text = file.read().splitlines()
    timestamp = int(text[0])
    
    buses = [int(i) for i in text[1].split(',') if not i.isalpha()]

next_deps = [(timestamp//i*i+i, i) for i in buses]

ans = min(next_deps)
print((ans[0]-timestamp)*ans[1])
