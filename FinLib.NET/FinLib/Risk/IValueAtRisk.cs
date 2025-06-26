namespace FinLib.Risk;

public interface IValueAtRisk
{
    double? ValueAtRisk(double confidence, double? initialInvestment = null);

    double? ValueAtRiskPercent(double confidence);

    double? ValueAtRiskAfterTime(double confidence, nint at, double? initialInvestment = null);
}