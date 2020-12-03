# AoC2020
Advent of Code 2020. Sadly I lost 2019 and 2018 :(
C++ solutions by Maksim Bjelić.

## Day 1
 - Smart way - 2-pointer algorithm O(N) and second part 3-pointer algorithm O(N^2)
 - Bruteforce way - brute force O(N^2) and O(N^3), but it doesn't matter input is small


<details>
  <summary>Spoiler warning</summary>
    Part 1: 1020084<br />
    Part 2: 295086480
</details>

Files:
```
1-smart.py  
1-stupid.py  
2-smart.py  
2-stupid.py
```
## Day 2
Go through each line and check if it is valid, nothing special here

<details>
  <summary>Spoiler warning</summary>
    Part 1: 439<br />
    Part 2: 584
</details>

Files:
```
1.py
2.py
1.cpp
2.cpp
```
## Day 3
Skip first y lines (y is steps down) with steps y like this:
```python
new_lines = lines[y::y]
```
Loop through `new_lines` and add x to counter (x is steps right, counter is current x coordinate)
```python
xcount += x
```
And next check if there is a tree
```python
current_spot = line[xcount % limit]
ans += 1 if current_spot == "#" else 0
```
And that should be it, everything else is easy.

<details>
  <summary>Spoiler warning</summary>
  Part 1: 299<br />
  Part 2: 3621285278
</details>

Files:
```
1.py
2.py
```
