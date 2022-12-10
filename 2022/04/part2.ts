const fs = require('fs');

fs.readFile('./input/input.txt', 'utf8', (err: any, data: string) => {
  if (err) {
    console.error(err);
    return;
  }

  function range(start: number, end: number): number[] {
    var ans: number[] = [];
    for (let i = start; i <= end; i++) {
        ans.push(i);
    }
    return ans;
  }


  function lineToSets(line: string): [Set<number>, Set<number>] {
    const getNumber = (a: number, b: number) => Number(line.split(",")[a].split("-")[b])
    return [new Set(range(getNumber(0,0), getNumber(0,1))), new Set(range(getNumber(1,0), getNumber(1,1)))]
  }

  function intersect(sets: [Set<number>, Set<number>]): Set<number> {
    return new Set([...sets[0]].filter(i => sets[1].has(i)))
  }

  function isOverlappingSet(sets: [Set<number>, Set<number>]): boolean {
    return intersect(sets).size > 0
  }

  function lineToIsOverlapping(line: string): boolean {
    return isOverlappingSet(lineToSets(line))
  }

  const lines = data.split("\n")
  const totalOverlapping = lines.map(line => lineToIsOverlapping(line)).filter(overlapping => overlapping).length

  console.log(totalOverlapping)
});

