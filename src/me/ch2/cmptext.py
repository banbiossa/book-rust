from pathlib import Path

root = Path(__file__).parent
afile = root / "fizzbuzz_python.txt"
bfile = root / "fizzbuzz_rust.txt"

with open(afile, "r") as f:
    a = f.read().strip()
with open(bfile, "r") as f:
    b = f.read().strip()

if a == b:
    print("OK")
else:
    print('ng')
