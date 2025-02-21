namespace FinLib;

public static class Indicators
{
    public static double RelativeStrengthIndicator(double timePeriod, double averageGain, double averageLoss) => NativeMethods.relative_strength_indicator(timePeriod, averageGain, averageLoss);
    public static double RelativeStrengthIndicatorSmoothed(double timePeriod, double previousAverageGain, double currentGain, double previousAverageLoss, double currentLoss) => NativeMethods.relative_strength_indicator_smoothed(timePeriod, previousAverageGain, currentGain, previousAverageLoss, currentLoss);
}