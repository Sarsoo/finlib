namespace FinLib;

public interface IProfit<T>: IPayoff<T>
{
    double Profit(T underlying);
}