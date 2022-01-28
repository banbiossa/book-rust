""" fizzbuzz for 1 to 100
fizz on 3
buzz on 5
fiizzbuzz on 15
"""

for i in range(1, 101):
    if i % 15 == 0:
        print("fizzbuzz")
        continue
    if i % 3 == 0:
        print('fizz')
        continue
    if i % 5 == 0:
        print('buzz')
        continue
    print(i)
