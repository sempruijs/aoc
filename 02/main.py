# Transform text to useful data
lines = []

file = open("input.txt", "r")

for line in file:
    lines.append(line.strip())
file.close()

# amount of valid passwords
valid_passwords_one = 0
valid_passwords_two = 0


def valid_password_one(password, letter, min, max):
    times_in_password = 0

    for character in list(password):
        if character == letter:
            times_in_password += 1

    return min <= times_in_password <= max


# ^ means A or B but not A and B
def valid_password_two(password, letter, min, max):
    return (list(password)[min - 1] == letter) ^ (list(password)[max - 1] == letter)


for line in lines:
    minmax = line.split(" ", 1)[0]
    min, max = minmax.split("-")
    min = int(min)
    max = int(max)

    letter = list(line.split()[1])[0]
    password = line.split()[2]

    if valid_password_one(password, letter, min, max):
        valid_passwords_one += 1
    if valid_password_two(password, letter, min, max):
        valid_passwords_two += 1


print(valid_passwords_one)
print(valid_passwords_two)