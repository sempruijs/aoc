const fs = require('fs');

fs.readFile('./input/input.txt', 'utf8', (err: any, data: string) => {
  if (err) {
    console.error(err);
    return;
  }

  function crateLinesToStacks(crateLines: string[]) {
    const transpose = (matrix: string[][]) => matrix[0].map((_, colIndex) => matrix.map(row => row[colIndex]))
    return transpose(crateLines.reverse().map(line => Array.from(lineWithoutAsciiArt(line)))).map(stack => stack.filter(char => char != " "))
  }

  function lineWithoutAsciiArt(line: string): string {
    const crateWidth = 3
    const amountOfStacks = (line.length + 1) / (crateWidth + 1)

    let lineWithoutAsciiArt = ""
    for (let i = 0; i < lines[0].length; i++) {
      if (i % (crateWidth + 1) == 1) {
        lineWithoutAsciiArt += line[i]
      }
    }
    return lineWithoutAsciiArt
  }


  function transform(stacks: string[][], from: number, to: number, times: number): string[][] {
    const fromIndex = from - 1
    const toIndex = to - 1    
    const movingCrates = stacks[fromIndex].slice(-times)
    let transformedStacks = stacks  

    // remove crates from old stack
    transformedStacks[fromIndex] = stacks[fromIndex].slice(0, stacks[fromIndex].length - times)

    // add crates to destanated stack
    transformedStacks[toIndex] = stacks[toIndex].concat(movingCrates)

    return transformedStacks
  }

  function applyTaskToStacks(stacks: string[][], task: string) {
    const from = Number(task.split(" ")[3])
    const to = Number(task.split(" ")[5])
    const times = Number(task.split(" ")[1])

    return transform(stacks, from, to, times)
  }

  function applyTasksToStacks(stacks: string[][], tasks: string[]): string[][] {
    return tasks.length == 1 ? applyTaskToStacks(stacks, tasks[0]) : applyTasksToStacks(applyTaskToStacks(stacks, tasks[0]), tasks.splice(1,tasks.length))
  }

  function showTopCrates(stacks: string[][]): string {
    return  stacks[0][stacks[0].length - 1] += stacks.length > 1 ? showTopCrates(stacks.splice(1, stacks.length)) : ""
  }


  const lines = data.split("\n")
  const crateLines = lines.filter(line => line.includes("["))
  const tasks = lines.filter(line => line.includes("move"))
  const topCrates = showTopCrates(applyTasksToStacks(crateLinesToStacks(crateLines), tasks))

  console.log(topCrates)
});

