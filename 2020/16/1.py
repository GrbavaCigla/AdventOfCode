with open("input", "r") as file:
    text = file.read().split('\n\n')

    raw_rules = text[0].splitlines()
    raw_ticket = text[1].splitlines()[1]
    raw_tickets = text[2].splitlines()[1:]

class Rule:
    def __init__(self, line):
        splited = line.split(':')[1].split()
        
        first = splited[0].split("-")
        second = splited[2].split("-")

        self.lower1 = int(first[0])
        self.lower2 = int(second[0])
        self.upper1 = int(first[1])
        self.upper2 = int(second[1])

    def is_valid(self, value):
        return (self.lower1 <= value <= self.upper1) or (self.lower2 <= value <= self.upper2)

rules = [Rule(i) for i in raw_rules]

ans = 0
for ticket in raw_tickets:
    values = list(map(int, ticket.split(',')))

    for value in values:
        valids = []
        
        for rule in rules:
            valids.append(not rule.is_valid(value))
        
        if all(valids):
            ans += value

print(ans)