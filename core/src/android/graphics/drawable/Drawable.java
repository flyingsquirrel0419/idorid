package android.graphics.drawable;
public abstract class Drawable {
    int resId;
    private int left, top, right, bottom, alpha=255;
    public void setAlpha(int a) { alpha=a; }
    public void setBounds(int l,int t,int r,int b) { left=l; top=t; right=r; bottom=b; }
    public void draw(android.graphics.Canvas canvas) { if (resId != 0) canvas.idoridDrawBitmap(resId,left,top,right,bottom,alpha); }
    public void setTint(int c) {}
    public int getIntrinsicWidth() { return -1; }
    public int getIntrinsicHeight() { return -1; }
    public Drawable mutate() { return this; }
    /** idorid: host color, or 0 when not a plain color. */
    public int idoridColor() { return 0; }
    public int idoridResource() { return resId; }
    public static Drawable fromResource(int id) { Drawable d = new Drawable() {}; d.resId = id; return d; }
}
