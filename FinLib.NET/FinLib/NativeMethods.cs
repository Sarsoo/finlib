namespace FinLib;

internal partial struct NullableFloat
{
    public static implicit operator double?(NullableFloat v) => v.is_valid ?  v.val : null;
    public static implicit operator NullableFloat(double? v) => v.HasValue ?
        new NullableFloat {is_valid = true, val = v.Value}
        : new NullableFloat {is_valid = false, val = 0.0};
}

internal partial struct Tuple
{
    public static implicit operator (double, double)?(Tuple v) => v.is_valid ? (v.one, v.two) : null;
}