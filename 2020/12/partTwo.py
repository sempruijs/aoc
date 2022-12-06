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
# rotation = 1
x = 0
y = 0
waypoint_x = 10
waypoint_y = -1


def rotate_waypoint(direction, amount):
    global waypoint_x
    global waypoint_y
    print (str(waypoint_x) + " " + str(waypoint_y))
    current_waypoint_x = waypoint_x
    current_waypoint_y = waypoint_y
    change_rotation = amount / 90
    waypoint_x = 0
    waypoint_y = 0
    if direction == "R":
        while change_rotation > 0:
            waypoint_x = -current_waypoint_y
            waypoint_y = current_waypoint_x
            change_rotation -= 1
    elif direction == "L":
        while change_rotation > 0:
            waypoint_x = current_waypoint_y
            if current_waypoint_x > 0 < current_waypoint_y or current_waypoint_x < 0 < current_waypoint_y:
                waypoint_y = -current_waypoint_x
            if current_waypoint_y < 0 > current_waypoint_x or current_waypoint_y > 0 > current_waypoint_x:
                waypoint_y = current_waypoint_x
            current_waypoint_x = waypoint_x
            current_waypoint_y = waypoint_y
            change_rotation -= 1
    print (str(waypoint_x) + " " + str(waypoint_y))


def forward(amount):
    global x
    global y
    x += waypoint_x * amount
    y += waypoint_y * amount


def follow_instruction(instruction, amount):
    global waypoint_x
    global waypoint_y
    if instruction == "N":
        waypoint_y -= amount
    elif instruction == "E":
        waypoint_x += amount
    elif instruction == "S":
        waypoint_y += amount
    elif instruction == "W":
        waypoint_x -= amount
    elif instruction == "F":
        forward(amount)
    else:
        rotate_waypoint(instruction, amount)
    # print (str(x) + " " + str(y))


for line in lines:
    follow_instruction(line[0], int(line[1:]))

if x < 0:
    x *= -1
elif y < 0:
    y *= -1

print(x + y)
