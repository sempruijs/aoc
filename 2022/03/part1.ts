const fs = require('fs');

fs.readFile('./input/example.txt', 'utf8', (err: any, data: string) => {
  if (err) {
    console.error(err);
    return;
  }
  const lines = data.split("\n")
  console.log(lines[0])
});

