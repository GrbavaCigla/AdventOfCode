import re

def simple_solve(expression):
    exp = expression.split()
    if len(exp) == 1:
        return int(expression)

    result = str(eval(exp[0]+exp[1]+exp[2]))
    next_exp = result + ' ' + ' '.join(exp[3:])

    return normal_solve(next_exp)

exp = '((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2'

sentences  = re.split(r'(?<=[a-zA-Z]{2,}\)?\.) |\(', exp)
print(sentences)