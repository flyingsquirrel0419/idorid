package android.animation;
public class ArgbEvaluator implements TypeEvaluator<Integer> {
    private static final ArgbEvaluator INSTANCE = new ArgbEvaluator();
    public static ArgbEvaluator getInstance() { return INSTANCE; }
    public Integer evaluate(float f, Integer start, Integer end) {
        int a = start.intValue(), b = end.intValue(), out = 0;
        for (int shift = 0; shift <= 24; shift += 8) {
            int x = (a >>> shift) & 255, y = (b >>> shift) & 255;
            out |= ((int)(x + f * (y - x) + 0.5f) & 255) << shift;
        }
        return Integer.valueOf(out);
    }
}
