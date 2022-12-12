const fs = require('fs');

fs.readFile('./input/input.txt', 'utf8', (err: any, data: string) => {
  if (err) {
    console.error(err);
    return;
  }

  // cheks if a string has a duplicate character
  function stringContainsDuplicate(string: string): boolean {
    const stringWithoutCheckedChar = string.slice(1, Array.from(string).length)
    return string.length == 2 ? stringWithoutCheckedChar.includes(string[0]) : stringWithoutCheckedChar.includes(string[0]) ? true : stringContainsDuplicate(stringWithoutCheckedChar)
  }

  function getStartMarker(line: string): number {
    const markerLength = 4
    for (let i = 0; i < Array.from(line).length; i++) {
      if (!stringContainsDuplicate(line.slice(i, i + markerLength))) {
        return i + markerLength
      }
    }
    return 0
  }

  const line = data
  console.log(getStartMarker(line))
});

