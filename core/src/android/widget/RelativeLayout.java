package android.widget;

import android.content.Context;
import android.util.AttributeSet;
import android.view.ViewGroup;

/** Approximated as a vertical stack (rules are ignored for now). */
public class RelativeLayout extends ViewGroup {
    public static final int TRUE = -1, LEFT_OF = 0, RIGHT_OF = 1, ABOVE = 2, BELOW = 3, ALIGN_PARENT_TOP = 10, ALIGN_PARENT_BOTTOM = 12, CENTER_IN_PARENT = 13, CENTER_HORIZONTAL = 14, CENTER_VERTICAL = 15, ALIGN_PARENT_LEFT = 9, ALIGN_PARENT_RIGHT = 11;
    public static class LayoutParams extends ViewGroup.MarginLayoutParams {
        public LayoutParams(int w, int h) { super(w, h); }
        public LayoutParams(ViewGroup.LayoutParams p) { super(p); }
        public LayoutParams(Context c, AttributeSet a) { super(c, a); }
        public void addRule(int verb) {}
        public void addRule(int verb, int subject) {}
        public void removeRule(int verb) {}
    }
    public RelativeLayout(Context c) { super(c); }
    public RelativeLayout(Context c, AttributeSet a) { super(c, a); }
    public RelativeLayout(Context c, AttributeSet a, int s) { super(c, a, s); }
    public void setGravity(int g) {}
    protected ViewGroup.LayoutParams generateDefaultLayoutParams() { return new LayoutParams(LayoutParams.WRAP_CONTENT, LayoutParams.WRAP_CONTENT); }
    public LayoutParams generateLayoutParams(AttributeSet attrs) { return new LayoutParams(getContext(), attrs); }
    protected ViewGroup.LayoutParams generateLayoutParams(ViewGroup.LayoutParams p) { return new LayoutParams(p); }
    protected boolean checkLayoutParams(ViewGroup.LayoutParams p) { return p instanceof LayoutParams; }
    protected String idoridType() { return "LinearLayout"; }
    protected void idoridProps(StringBuilder sb) { android.view.Json.key(sb, "orientation"); sb.append("\"vertical\""); super.idoridProps(sb); }
}
