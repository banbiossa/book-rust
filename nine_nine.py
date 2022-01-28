"""9*9 の表を作る
"""
for i in range(1, 10):
    for j in range(1, 10):
        letter = str(i * j).rjust(2)
        print(f"{letter},", end=' ')
    print("")