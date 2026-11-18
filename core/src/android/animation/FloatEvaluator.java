package android.animation;
public class FloatEvaluator implements TypeEvaluator<Number> {
    public Float evaluate(float f, Number a, Number b) { return Float.valueOf(a.floatValue() + f * (b.floatValue() - a.floatValue())); }
}
