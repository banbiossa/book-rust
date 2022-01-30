from collections import Counter

data = "C,C,A,A,A,B,C,C,B,B,B,C,B"

c = Counter(data.split(","))
print(c)
