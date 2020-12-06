with open("input", "r") as file:
    text = file.read().split("\n\n")
    text = [i.replace("\n", "") for i in text]
    text = [list(set(i)) for i in text]
    text = [len(i) for i in text]

print(sum(text))