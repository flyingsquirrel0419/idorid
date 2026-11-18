package android.widget;

import android.content.Context;
import android.util.AttributeSet;
import android.view.ViewGroup;
import android.view.XmlAttrs;

public class FrameLayout extends ViewGroup {
    public static class LayoutParams extends ViewGroup.MarginLayoutParams {
        public int gravity = -1;
        public LayoutParams(int w, int h) { super(w, h); }
        public LayoutParams(int w, int h, int gravity) { super(w, h); this.gravity = gravity; }
        public LayoutParams(ViewGroup.LayoutParams p) { super(p); }
        public LayoutParams(Context c, AttributeSet attrs) {
            super(c, attrs);
            if (attrs instanceof XmlAttrs) gravity = ((XmlAttrs) attrs).integer("layout_gravity", -1);
        }
    }
    public FrameLayout(Context c) { super(c); }
    public FrameLayout(Context c, AttributeSet a) { super(c, a); }
    public FrameLayout(Context c, AttributeSet a, int s) { super(c, a, s); }
    public FrameLayout(Context c, AttributeSet a, int s, int r) { super(c, a, s, r); }
    protected ViewGroup.LayoutParams generateDefaultLayoutParams() { return new LayoutParams(LayoutParams.MATCH_PARENT, LayoutParams.MATCH_PARENT); }
    public LayoutParams generateLayoutParams(AttributeSet attrs) { return new LayoutParams(getContext(), attrs); }
    protected ViewGroup.LayoutParams generateLayoutParams(ViewGroup.LayoutParams p) { return new LayoutParams(p); }
    protected boolean checkLayoutParams(ViewGroup.LayoutParams p) { return p instanceof LayoutParams; }
    public void setForeground(android.graphics.drawable.Drawable d) {}
    protected String idoridType() { return "FrameLayout"; }
}
