with open("input", "r") as file:
    text = file.read().split("\n\n")
    text = [i.splitlines() for i in text]

res = 0
for group in text:
    main_set = set(group[0])
    for people in group[1:]:
        main_set = main_set.intersection(set(people))

    res += len(main_set)

print(res)