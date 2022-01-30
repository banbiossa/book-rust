import random

nums = list(range(1, 75 + 1))
random.shuffle(nums)
nums[12] = "  *"

for y in range(5):
    for x in range(5):
        print(f"{nums[y*5+x]:3}", end="")
    print("")