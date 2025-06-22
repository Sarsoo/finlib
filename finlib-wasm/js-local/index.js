import {Curve, CurveType, init_logging} from "finlib";

init_logging();

// console.log(ValueAtRisk.varcovar([1, 2, 3, 4], 0.1));
// console.log(ValueAtRisk.varcovar([1, 2, 3, 4], 0.05));
//
// let portfolio = new Portfolio([new PortfolioAsset("test", 1, [1.0, 2.0, 3.0])]);
// console.log(portfolio.isValid());
// console.log(portfolio.valueAtRisk(0.1, 1000000));

let curve = new Curve(CurveType.Absolute);
curve.addRateFrom(100, 100, new Date(2024, 1, 1));
curve.addRateFrom(100, 100, new Date(2024, 1, 2));
curve.addRateFrom(100, 100, new Date(2024, 1, 3));
curve.addRateFrom(100, 100, new Date(2024, 1, 4));
console.log(curve.length);
console.log(curve.getRate(new Date(2024, 1, 4)).bid);
