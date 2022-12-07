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

  function matchResultToOption(opponent: Option, result: MatchResult): Option {
    if (result == MatchResult.Draw) {
      return opponent
    } else if (result == MatchResult.Win) {
      return opponent == Option.Rock ? Option.Paper : opponent == Option.Paper ? Option.Scissors : Option.Rock
    } else {
      return opponent == Option.Rock ? Option.Scissors : opponent == Option.Paper ? Option.Rock : Option.Paper
    }
  }

  function charToOption(char: string): Option {
    return char == "A" ? Option.Rock : char == "B" ? Option.Paper : Option.Scissors
  }

  function charToMatchResult(char: string): MatchResult {
    return char == "X" ? MatchResult.Lose : char == "Y" ? MatchResult.Draw : MatchResult.Win
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
  const matchesResults = lines.map((line) => charToMatchResult(line.charAt(2)))

  let yourOptions: Array<Option> = []

  for (let i = 0; i <matchesResults.length; i++) {
    yourOptions.push(matchResultToOption(opponentOptions[i], matchesResults[i]))
  }

  let matchesPoints: Array<number> = []

  for (let i = 0; i < yourOptions.length; i++) {
    matchesPoints.push(getMatchPoints(opponentOptions[i], yourOptions[i]))
  }
  
  const totalPoints = matchesPoints.reduce((previous, current) => previous + current)
  console.log(totalPoints)
});

