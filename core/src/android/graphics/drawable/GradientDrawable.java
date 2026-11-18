package android.graphics.drawable;
public class GradientDrawable extends Drawable {
    public static final int RECTANGLE = 0, OVAL = 1;
    private int color;
    private float radius;
    public GradientDrawable() {}
    public void setColor(int c) { color = c; }
    public void setCornerRadius(float r) { radius = r; }
    public void setShape(int s) {}
    public void setStroke(int w, int c) {}
    public int idoridColor() { return color; }
    public float idoridRadius() { return radius; }
}
