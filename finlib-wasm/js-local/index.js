import { ValueAtRisk, Portfolio, PortfolioAsset } from "finlib";

console.log(ValueAtRisk.varcovar([1, 2, 3, 4], 0.1));
console.log(ValueAtRisk.varcovar([1, 2, 3, 4], 0.05));

let portfolio = new Portfolio([new PortfolioAsset(1.0, "test", [1.0, 2.0, 3.0])]);
console.log(portfolio.isValid());
console.log(portfolio.valueAtRisk(0.1));
