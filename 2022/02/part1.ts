const fs = require('fs');

fs.readFile('./input/input.txt', 'utf8', (err: any, data: string) => {
  if (err) {
    console.error(err);
    return;
  }
  enum Option {
    Rock,
    Paper,
    Scissors
  }

  enum MatchResult {
    Win,
    Draw,
    Lose
  }

  function charToOption(char: string): Option {
    switch (char) {
      case "A":
      case "X":
        return Option.Rock
      case "B":
      case "Y":
        return Option.Paper
      default:
        return Option.Scissors
    }
  }

  function getMatchResult(opponent: Option, you: Option): MatchResult {
    if (opponent == you) {
      return MatchResult.Draw
    } else if (opponent == Option.Rock) {
      return you == Option.Paper ? MatchResult.Win : MatchResult.Lose
    } else if (opponent == Option.Paper) {
      return you == Option.Scissors ? MatchResult.Win : MatchResult.Lose
    } else {
      return you == Option.Rock ? MatchResult.Win : MatchResult.Lose
    }
  }

  function matchResultToPoints(result: MatchResult): number {
    return result == MatchResult.Win ? 6 : result == MatchResult.Draw ? 3 : 0
  }

  function getOptionPoints(option: Option): number {
    return option == Option.Rock ? 1 : option == Option.Paper ? 2 : 3
  }

  function getMatchPoints(opponent: Option, you: Option): number {
    return matchResultToPoints(getMatchResult(opponent, you)) + getOptionPoints(you)
  }

  const lines = data.split("\n")  
  const opponentOptions = lines.map((line) => charToOption(line.charAt(0)))
  const yourOptions = lines.map((line) => charToOption(line.charAt(2)))

  let matchesPoints: Array<number> = []

  for (let i = 0; i < yourOptions.length; i++) {
    matchesPoints.push(getMatchPoints(opponentOptions[i], yourOptions[i]))
  }
  const totalPoints = matchesPoints.reduce((previous, current) => previous + current)
  console.log(totalPoints)

});

