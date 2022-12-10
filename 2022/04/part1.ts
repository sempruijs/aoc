const fs = require('fs');

fs.readFile('./input/input.txt', 'utf8', (err: any, data: string) => {
  if (err) {
    console.error(err);
    return;
  }

  function lineToTuples(line: string): Array<[number, number]> {
    const tupleNumber = (a: number, b: number) => Number(line.split(",")[a].split("-")[b])
    return [[tupleNumber(0,0), tupleNumber(0,1)], [tupleNumber(1,0), tupleNumber(1,1)]]
  }

  function tuplesContainUseless(tuples: Array<[number, number]>): boolean {
    return (tuples[0][0] >= tuples[1][0] && tuples[0][1] <= tuples[1][1]) || (tuples[0][0] <= tuples[1][0] && tuples[0][1] >= tuples[1][1])
  }

  const lines = data.split("\n")
  const uselessJobsAmount = lines.map(line => tuplesContainUseless(lineToTuples(line))).filter(useless => useless).length
  console.log(uselessJobsAmount)

});

