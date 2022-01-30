""" fibonnaci
"""

end = 10
a = 1
b = 1
print(a)
for i in range(end):
    a += b
    a, b = b, a
    print(a)