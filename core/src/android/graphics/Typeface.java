package android.graphics;
public class Typeface {
    public static final int NORMAL = 0, BOLD = 1, ITALIC = 2, BOLD_ITALIC = 3;
    public static final Typeface DEFAULT = new Typeface(0), DEFAULT_BOLD = new Typeface(1), MONOSPACE = new Typeface(0), SANS_SERIF = new Typeface(0), SERIF = new Typeface(0);
    private final int style;
    private Typeface(int s) { style = s; }
    public int getStyle() { return style; }
    public boolean isBold() { return (style & BOLD) != 0; }
    public static Typeface create(Typeface f, int style) { return new Typeface(style); }
    public static Typeface defaultFromStyle(int style) { return new Typeface(style); }
}
