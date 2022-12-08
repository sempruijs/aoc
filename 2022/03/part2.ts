const fs = require('fs');

fs.readFile('./input/input.txt', 'utf8', (err: any, data: string) => {
  if (err) {
    console.error(err);
    return;
  }

  function chunks<T>(arr, chunkSize): Array<T> {
    const res: Array<T> = [];
    for (let i = 0; i < arr.length; i += chunkSize) {
        const chunk = arr.slice(i, i + chunkSize);
        res.push(chunk);
    }
    return res;
}

  function getMatchingChar(a: string, b: string): string {
    for (let i = 0; i < a.length; i++) {
      if (b.includes(a.charAt(i))) {
        return a.charAt(i)
      }
    }
    return "Error finding character"
  }

  function getCompartments(line: string): Array<string> {
    const compartmentLength = line.length / 2
    return [line.slice(0, compartmentLength), line.slice(-compartmentLength)]
  }

  function charValue(char: string): number {
    return (parseInt(char, 36) - 9) + (char == char.toUpperCase() ? 26 : 0)
  }

  function lineToValue(line: string): number {
    return charValue(getMatchingChar(getCompartments(line)[0], getCompartments(line)[1]))
  }
  const lines = data.split("\n")
  const totalPoints = lines.map((line) => lineToValue(line)).reduce((previous, current) => previous + current)
  console.log(totalPoints)
});

