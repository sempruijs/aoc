const fs = require('fs');

fs.readFile('./input/input.txt', 'utf8', (err: any, data: string) => {
  if (err) {
    console.error(err);
    return;
  }

  function chunks(arr: Array<string>, chunkSize: number): Array<Array<string>> {
    const res: Array<Array<string>> = [];
    for (let i = 0; i < arr.length; i += chunkSize) {
        const chunk = arr.slice(i, i + chunkSize);
        res.push(chunk);
    }
    return res;
}

  function getMatchingChar(group: Array<string>) {
    const a = stringToSet(group[0])
    const b = stringToSet(group[1])
    const c = stringToSet(group[2])

    return [...intersectSets(a, intersectSets(b, c))][0]
  }

  function stringToSet(input: string): Set<string> {
    return new Set(Array.from(input))
  }

  function intersectSets(a: Set<string>, b: Set<string>): Set<string> {
    return new Set([...a].filter(i => b.has(i)))
  }


  function charValue(char: string): number {
    return (parseInt(char, 36) - 9) + (char == char.toUpperCase() ? 26 : 0)
  }

  function groupToValue(group: Array<string>): number {
    return charValue(getMatchingChar(group))
  }

  const lines = data.split("\n")
  const groups = chunks(lines, 3)
  const totalPoints = groups.map((group => groupToValue(group))).reduce((p, c) => p + c)

  console.log(totalPoints)
});

