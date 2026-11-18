package android.graphics;
public class Typeface {
    public static final int NORMAL = 0, BOLD = 1, ITALIC = 2, BOLD_ITALIC = 3;
    public static final Typeface DEFAULT = new Typeface(0), DEFAULT_BOLD = new Typeface(1), MONOSPACE = new Typeface(0), SANS_SERIF = new Typeface(0), SERIF = new Typeface(0);
    private final int style;
    private Typeface(int s) { style = s; }
    public int getStyle() { return style; }
    public int getWeight() { return isBold()?700:400; } public boolean isItalic() { return (style & ITALIC)!=0; } public static Typeface create(String family,int style) { return new Typeface(style); } public static Typeface create(Typeface family,int weight,boolean italic) { return new Typeface((weight>=600?BOLD:0)|(italic?ITALIC:0)); } public static Typeface createFromFile(java.io.File file) { return DEFAULT; } public static Typeface createFromFile(String file) { return DEFAULT; } public static class Builder { public Builder(String s) {} public Builder(java.io.File f) {} public Builder setWeight(int w) { return this; } public Builder setItalic(boolean b) { return this; } public Typeface build() { return DEFAULT; } } public static class CustomFallbackBuilder { public CustomFallbackBuilder(android.graphics.fonts.FontFamily family) {} public CustomFallbackBuilder setStyle(android.graphics.fonts.FontStyle style) { return this; } public CustomFallbackBuilder setSystemFallback(String name) { return this; } public Typeface build() { return DEFAULT; } } public boolean isBold() { return (style & BOLD) != 0; }
    public static Typeface create(Typeface f, int style) { return new Typeface(style); }
    public static Typeface defaultFromStyle(int style) { return new Typeface(style); }
}
