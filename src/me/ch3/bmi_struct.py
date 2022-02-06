height_cm = float(input("Enter your height in cm: "))
weight_kg = float(input("Enter your weight in kg: "))

height = height_cm / 100
bmi = weight_kg / (height * height)

bmi_list = [{
    "min": 0,
    "max": 18.5,
    "status": "underweight"
}, {
    "min": 18.5,
    "max": 25,
    "status": "normal"
}, {
    "min": 25,
    "max": 30,
    "status": "overweight"
}, {
    "min": 30,
    "max": 40,
    "status": "obese"
}, {
    "min": 40,
    "max": float("inf"),
    "status": "morbidly obese"
}]

for range in bmi_list:
    if range["min"] <= bmi < range["max"]:
        print(f"Your BMI is {bmi:.2f} which means you are", range["status"])
        break