package android.graphics.drawable;
public abstract class Drawable {
    int resId;
    public void setAlpha(int a) {}
    public void setTint(int c) {}
    public int getIntrinsicWidth() { return -1; }
    public int getIntrinsicHeight() { return -1; }
    public Drawable mutate() { return this; }
    /** idorid: host color, or 0 when not a plain color. */
    public int idoridColor() { return 0; }
    public int idoridResource() { return resId; }
    public static Drawable fromResource(int id) { Drawable d = new Drawable() {}; d.resId = id; return d; }
}
