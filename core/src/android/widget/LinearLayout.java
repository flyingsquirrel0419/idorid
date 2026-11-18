package android.widget;

import android.content.Context;
import android.util.AttributeSet;
import android.view.Gravity;
import android.view.Json;
import android.view.ViewGroup;
import android.view.XmlAttrs;

public class LinearLayout extends ViewGroup {
    public static final int HORIZONTAL = 0, VERTICAL = 1;
    public static final int SHOW_DIVIDER_NONE = 0, SHOW_DIVIDER_MIDDLE = 2;
    public static class LayoutParams extends ViewGroup.MarginLayoutParams {
        public float weight;
        public int gravity = -1;
        public LayoutParams(int w, int h) { super(w, h); }
        public LayoutParams(int w, int h, float weight) { super(w, h); this.weight = weight; }
        public LayoutParams(ViewGroup.LayoutParams p) { super(p); }
        public LayoutParams(Context c, AttributeSet attrs) {
            super(c, attrs);
            XmlAttrs a = attrs instanceof XmlAttrs ? (XmlAttrs) attrs : XmlAttrs.EMPTY;
            weight = a.dimension("layout_weight", 0);
            String g = a.raw("layout_gravity");
            if (g != null) gravity = a.integer("layout_gravity", -1);
        }
    }
    private int orientation = HORIZONTAL;
    private int gravity;
    public LinearLayout(Context c) { super(c); }
    public LinearLayout(Context c, AttributeSet a) { this(c, a, 0); }
    public LinearLayout(Context c, AttributeSet a, int s) {
        super(c, a, s);
        if (a instanceof XmlAttrs) {
            XmlAttrs x = (XmlAttrs) a;
            String o = x.string("orientation");
            orientation = o != null && (o.equals("vertical") || o.equals("1")) ? VERTICAL : HORIZONTAL;
            gravity = x.integer("gravity", 0);
        }
    }
    public void setOrientation(int o) { orientation = o; }
    public int getOrientation() { return orientation; }
    public void setGravity(int g) { gravity = g; }
    public int getGravity() { return gravity; }
    public void setWeightSum(float w) {}
    public void setBaselineAligned(boolean b) {}
    public void setShowDividers(int s) {}
    public void setDividerDrawable(android.graphics.drawable.Drawable d) {}
    protected ViewGroup.LayoutParams generateDefaultLayoutParams() {
        return orientation == HORIZONTAL ? new LayoutParams(LayoutParams.WRAP_CONTENT, LayoutParams.WRAP_CONTENT) : new LayoutParams(LayoutParams.MATCH_PARENT, LayoutParams.WRAP_CONTENT);
    }
    public LayoutParams generateLayoutParams(AttributeSet attrs) { return new LayoutParams(getContext(), attrs); }
    protected ViewGroup.LayoutParams generateLayoutParams(ViewGroup.LayoutParams p) { return new LayoutParams(p); }
    protected boolean checkLayoutParams(ViewGroup.LayoutParams p) { return p instanceof LayoutParams; }
    protected String idoridType() { return "LinearLayout"; }
    protected void idoridProps(StringBuilder sb) {
        Json.key(sb, "orientation"); Json.str(sb, orientation == VERTICAL ? "vertical" : "horizontal");
        String g = Gravity.describe(gravity);
        if (g != null) { Json.key(sb, "gravity"); Json.str(sb, g); }
        super.idoridProps(sb);
    }
}
