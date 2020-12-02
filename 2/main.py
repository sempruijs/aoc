# Transform text to useful data
data = []

file = open("input.txt", "r")

for line in file:
    data.append(line.strip())
file.close()

