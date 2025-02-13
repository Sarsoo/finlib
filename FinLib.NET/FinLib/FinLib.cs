namespace FinLib;

public static class FinLib
{
    public static ulong Add(ulong a, ulong b)
    {
        return NativeMethods.add(a, b);
    }
}