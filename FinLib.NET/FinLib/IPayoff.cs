namespace FinLib;

public interface IPayoff<T>
{
    double Payoff(T underlying);
}