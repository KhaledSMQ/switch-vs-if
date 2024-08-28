console.time("if-else (int)");
let x = 7;
let result;
const numTests = 100000000;

for (let i = 0; i < numTests; i++) {
  if (x === 1) result = 1;
  else if (x === 2) result = 2;
  else if (x === 3) result = 3;
  else if (x === 4) result = 4;
  else if (x === 5) result = 5;
  else if (x === 6) result = 6;
  else if (x === 7) result = 7;
  else if (x === 8) result = 8;
  else if (x === 9) result = 9;
}
console.timeEnd("if-else (int)");

console.time("switch-case (int)");

for (let i = 0; i < numTests; i++) {
  switch (x) {
    case 1:
      result = 1;
      break;
    case 2:
      result = 2;
      break;
    case 3:
      result = 3;
      break;
    case 4:
      result = 4;
      break;
    case 5:
      result = 5;
      break;
    case 6:
      result = 6;
      break;
    case 7:
      result = 7;
      break;
    case 8:
      result = 8;
      break;
    case 9:
      result = 9;
      break;
  }
}
console.timeEnd("switch-case (int)");

console.time("if-else (string)");
let str = "seven";

for (let i = 0; i < numTests; i++) {
  if (str === "one") result = 1;
  else if (str === "two") result = 2;
  else if (str === "three") result = 3;
  else if (str === "four") result = 4;
  else if (str === "five") result = 5;
  else if (str === "six") result = 6;
  else if (str === "seven") result = 7;
  else if (str === "eight") result = 8;
  else if (str === "nine") result = 9;
}
console.timeEnd("if-else (string)");

console.time("switch-case (string)");

for (let i = 0; i < numTests; i++) {
  switch (str) {
    case "one":
      result = 1;
      break;
    case "two":
      result = 2;
      break;
    case "three":
      result = 3;
      break;
    case "four":
      result = 4;
      break;
    case "five":
      result = 5;
      break;
    case "six":
      result = 6;
      break;
    case "seven":
      result = 7;
      break;
    case "eight":
      result = 8;
      break;
    case "nine":
      result = 9;
      break;
  }
}
console.timeEnd("switch-case (string)");
