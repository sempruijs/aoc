# Transform text to useful data
lines = []

file = open("input.txt", "r")

for line in file:
    lines.append(line.strip())
file.close()

# Variables
trees_encountert = 0
current_position = 0


def encounter_tree(line, position):
    return list(line)[position] == "#"


def calculated_position(position):
    if position <= 27:
        position += 3
    elif position == 28:
        position = 0
    elif position == 29:
        position = 1
    elif position == 30:
        position = 2

    return position


for line in lines:
    if encounter_tree(line, current_position):
        trees_encountert += 1

    current_position = calculated_position(current_position)

print(trees_encountert)
