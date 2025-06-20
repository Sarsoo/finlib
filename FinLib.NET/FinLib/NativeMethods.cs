namespace FinLib;

internal partial struct NullableFloat
{
    public static implicit operator double?(NullableFloat v) => v.is_valid ?  v.val : null;
}

internal partial struct Tuple
{
    public static implicit operator (double, double)?(Tuple v) => v.is_valid ? (v.one, v.two) : null;
}