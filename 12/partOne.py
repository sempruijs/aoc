# Transform text to useful data
lines = []

file = open("input.txt", "r")

for line in file:
    lines.append(line.strip())
file.close()

# Variables
# 0 is north, 1 is east, 2 is south and 3 is west
#    0
# 3     1
#    2
rotation = 1
x = 0
y = 0


def rotate(direction, amount):
    global rotation
    change_rotation = amount / 90
    if direction == "R":
        while change_rotation > 0:
            if rotation < 3:
                rotation += 1
            else:
                rotation = 0
            change_rotation -= 1
    elif direction == "L":
        while change_rotation > 0:
            if rotation > 0:
                rotation -= 1
            else:
                rotation = 3
            change_rotation -= 1


def forward(amount):
    global rotation
    global x
    global y
    if rotation == 0:
        y -= amount
    elif rotation == 1:
        x += amount
    elif rotation == 2:
        y += amount
    elif rotation == 3:
        x -= amount


def follow_instruction(instruction, amount):
    global x
    global y
    if instruction == "N":
        y -= amount
    elif instruction == "E":
        x += amount
    elif instruction == "S":
        y += amount
    elif instruction == "W":
        x -= amount
    elif instruction == "F":
        forward(amount)
    else:
        rotate(instruction, amount)


for line in lines:
    follow_instruction(line[0], int(line[1:]))

if x < 0:
    x *= -1
elif y < 0:
    y *= -1

print(x + y)
