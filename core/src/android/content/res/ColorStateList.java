package android.content.res;
public class ColorStateList {
    private final int color;
    public ColorStateList(int[][] states, int[] colors) { color = colors.length > 0 ? colors[colors.length - 1] : 0; }
    private ColorStateList(int c) { color = c; }
    public static ColorStateList valueOf(int c) { return new ColorStateList(c); }
    public int getDefaultColor() { return color; }
}
