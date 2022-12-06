# Transform text to useful data
lines = []

file = open("input.txt", "r")

for line in file:
    lines.append(line.strip())
file.close()

# Variables
total_trees_encountered = 0
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


def loop_trough_lines(right, down):
    global current_position
    current_position = 0
    index = 0
    trees_encountert = 0
    for line in lines:
        if index % down == 0:
            if encounter_tree(line, current_position):
                trees_encountert += 1

            current_position = calculated_position(right)
        index += 1
    return trees_encountert


total_trees_encountered = loop_trough_lines(1, 1) * loop_trough_lines(3, 1) * loop_trough_lines(5, 1) * loop_trough_lines(7, 1) * loop_trough_lines(1, 2)

print(total_trees_encountered)