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


def calculated_position(right):
    counter = current_position
    right_used = right
    while right_used > 0:
        if counter == 30:
            counter = 0
            right_used -= 1
        else:
            counter += 1
            right_used -= 1
    return counter


for line in lines:
    if encounter_tree(line, current_position):
        trees_encountert += 1

    current_position = calculated_position(3)

print(trees_encountert)