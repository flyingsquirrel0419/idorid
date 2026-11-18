package android.widget;
import android.content.Context;
import android.util.AttributeSet;
import android.view.Json;
import android.view.View;
import android.view.XmlAttrs;
public class ImageView extends View {
    public enum ScaleType { MATRIX, FIT_XY, FIT_START, FIT_CENTER, FIT_END, CENTER, CENTER_CROP, CENTER_INSIDE }
    private int src;
    private int tint;
    private ScaleType scale = ScaleType.FIT_CENTER;
    public ImageView(Context c) { super(c); }
    public ImageView(Context c, AttributeSet a) { this(c, a, 0); }
    public ImageView(Context c, AttributeSet a, int s) {
        super(c, a, s);
        if (a instanceof XmlAttrs) { XmlAttrs x = (XmlAttrs) a; src = x.integer("src", x.integer("srcCompat", 0)); }
    }
    public void setImageResource(int id) { src = id; }
    public void setImageDrawable(android.graphics.drawable.Drawable d) { src = d == null ? 0 : d.idoridResource(); }
    public void setImageBitmap(android.graphics.Bitmap b) {}
    public void setScaleType(ScaleType s) { scale = s; }
    public ScaleType getScaleType() { return scale; }
    public void setColorFilter(int c) { tint = c; }
    public void setImageTintList(android.content.res.ColorStateList c) { tint = c == null ? 0 : c.getDefaultColor(); }
    public void setAdjustViewBounds(boolean b) {}
    protected String idoridType() { return "ImageView"; }
    protected void idoridProps(StringBuilder sb) {
        if (src != 0) {
            Json.key(sb, "src"); sb.append(src);
            String n = null;
            try { n = getResources().getResourceName(src); } catch (RuntimeException e) {}
            if (n != null) { Json.key(sb, "srcName"); Json.str(sb, n); }
        }
        if (tint != 0) { Json.key(sb, "tint"); Json.str(sb, android.graphics.Color.toHex(tint)); }
    }
}
