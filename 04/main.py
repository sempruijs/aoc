# Transform text to useful data
lines = []

file = open("input.txt", "r")

for line in file:
    lines.append(line.strip())
file.close()

empty_lines = 0
for line in lines:
    if line == "":
        empty_lines += 1

passports = []
passport_string = ""
valid_passports = 0

for line in lines:
    if line != "":
        if passport_string != "":
            passport_string += " " + line
        else:
            passport_string += line
    else:
        passports.append(passport_string)
        passport_string = ""

# Add last passport
passports.append(passport_string)


def valid_passport(passport):
    passport_data = passport.split(" ")

    passport_subjects = []
    for data in passport_data:
        passport_subjects.append(data.split(":")[0].strip())

    valid_subjects = {"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"}

    missing_subjects = valid_subjects - set(passport_subjects)

    return len(missing_subjects) == 0


for passport in passports:
    if valid_passport(passport):
        valid_passports += 1

print(str(valid_passports))
