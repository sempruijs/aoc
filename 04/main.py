# Transform text to useful data
lines = []

file = open("input.txt", "r")

for line in file:
    lines.append(line.strip())
file.close()

passports = []
passport_string = ""

for line in lines:
    if line != "":
        if passport_string != "":
            passport_string += " " + line
        else:
            passport_string += line
    else:
        passports.append(passport_string)
        passport_string = ""

print (passports)