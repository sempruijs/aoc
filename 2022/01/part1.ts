const fs = require('fs');

fs.readFile('./input/input.txt', 'utf8', (err: any, data: string) => {
  if (err) {
    console.error(err);
    return;
  }
  const lines = data.split("\n")  

  let evleValue = 0
  let elvesValues = []

  for (let i = 0; i < lines.length; i++) {
    if (lines[i] != "") {
      evleValue += Number(lines[i])
    } else {
      elvesValues.push(evleValue)
      evleValue = 0
    }
  }
  const sortedEvlesValues = elvesValues.sort((n1,n2) => n1 - n2)
  console.log(sortedEvlesValues[sortedEvlesValues.length - 1])
});

