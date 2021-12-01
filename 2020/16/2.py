from pprint import pprint

with open("input", "r") as file:
    text = file.read().split('\n\n')

    raw_rules = text[0].splitlines()
    raw_ticket = text[1].splitlines()[1].split(',')
    raw_tickets = text[2].splitlines()[1:]

    raw_tickets = [i.split(',') for i in raw_tickets]

def column(matrix, i):
    return [int(row[i]) for row in matrix]

def is_ticket_valid(ticket):
    for value in ticket:
        valids = []
        
        for rule in rules:
            valids.append(not rule.is_valid(int(value)))
        
        if all(valids):
            return False
    
    return True

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

result = {}
def match_cols(possible):
    if not any(possible.values()):
        return None

    for k, v in possible.items():
        if len(v) == 1:
            to_remove = v[0]
            result[v[0]] = k

    for k, v in possible.items():
        try:
            v.remove(to_remove)
        except:
            pass
        possible[k] = v

    return match_cols(possible)

rules = [Rule(i) for i in raw_rules]

raw_tickets = [i for i in raw_tickets if is_ticket_valid(i)]
columns = [column(raw_tickets, i) for i in range(len(raw_tickets[0]))]

possible = {k: [] for k in range(len(columns))}

for c, column in enumerate(columns):
    for r, rule in enumerate(rules):
        valids = []
        
        for field in column:
            valids.append(rule.is_valid(field))
        
        if all(valids):
            possible[c].append(r)

match_cols(possible)

ans = 1
for i in range(6):
    ans *= int(raw_ticket[result[i]])

print(ans)