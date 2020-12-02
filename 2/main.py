# Transform text to useful data
lines = []

file = open("input.txt", "r")

for line in file:
    lines.append(line.strip())
file.close()

# amount of valid passwords
valid_passwords = 0


def valid_password(password, letter, min, max):
    times_in_password = 0

    for character in list(password):
        if character == letter:
            times_in_password += 1

    return min <= times_in_password <= max


for line in lines:
    minmax = line.split(" ", 1)[0]
    min = int(minmax.split("-")[0])
    max = int(minmax.split("-")[1])

    letter = list(line.split()[1])[0]
    password = line.split()[2]

    if valid_password(password, letter, min, max):
        valid_passwords += 1

print(valid_passwords)