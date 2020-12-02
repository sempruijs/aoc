# Transform text to useful data
data = []

file = open("input.txt", "r")

for line in file:
    data.append(line.strip())
file.close()


def valid_password(password, letter, min, max):
    times_in_password = 0

    print(list(password))

    for character in list(password):
        if character == letter:
            times_in_password += 1

    return min <= times_in_password <= max