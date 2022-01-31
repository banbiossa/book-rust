import sys

dicfile = "./resources/ejdict-hand-utf8.txt"
if len(sys.argv) < 2:
    print("Usage: {} <word>".format(sys.argv[0]))
    quit()

word = sys.argv[1]
with open(dicfile, 'rt') as f:
    while True:
        line = f.readline()
        if not line:
            break
        if word in line:
            print(line.strip())