package android.animation;
public class IntEvaluator implements TypeEvaluator<Integer> {
    public Integer evaluate(float f, Integer a, Integer b) { return Integer.valueOf(a.intValue() + (int)(f * (b.intValue() - a.intValue()))); }
}
