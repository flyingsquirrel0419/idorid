package android.graphics;
public final class Bitmap {
    public enum Config { ALPHA_8, RGB_565, ARGB_4444, ARGB_8888 }
    private final int w, h;
    private Bitmap(int w, int h) { this.w = w; this.h = h; }
    public static Bitmap createBitmap(int w, int h, Config c) { return new Bitmap(w, h); }
    public int getWidth() { return w; }
    public int getHeight() { return h; }
    public void recycle() {}
}
