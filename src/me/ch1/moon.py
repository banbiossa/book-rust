distance = 384_400
car = 80
train = 300
day_in_hours = 24


def by(vehicle):
    return distance / (vehicle * day_in_hours)


print(f"{by(car)=}")
print(f"{by(train)=}")
