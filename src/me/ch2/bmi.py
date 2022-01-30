height_cm = float(input("身長は?(cm) "))
weight = float(input("体重は?(kg) "))

# bmi
height = height_cm / 100
bmi = weight / (height * height)
print(f"BMIは{bmi:.2f}です")

if bmi < 18.5:
    print("痩せ型")
elif bmi < 25:
    print("普通")
elif bmi < 30:
    print("肥満(軽)")
elif bmi < 35:
    print("肥満(中)")
elif bmi < 40:
    print("肥満(重)")
else:
    print("肥満(極)")
