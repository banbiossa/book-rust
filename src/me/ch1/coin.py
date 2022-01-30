""" 
3950
500: 10
100: 3
50: 10
"""

num_coins = {
    500: 10,
    100: 3,
    50: 10,
}

all_found = []
for i in range(11):
    for j in range(4):
        for k in range(11):
            if i * 500 + j * 100 + k * 50 == 3950:
                all_found.append((i, j, k))

print(len(all_found))
print(all_found)