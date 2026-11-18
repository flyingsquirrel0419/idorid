package android.widget;
import android.content.Context;
import android.util.AttributeSet;
public class ScrollView extends FrameLayout {
    public ScrollView(Context c) { super(c); }
    public ScrollView(Context c, AttributeSet a) { super(c, a); }
    public ScrollView(Context c, AttributeSet a, int s) { super(c, a, s); }
    public void fullScroll(int dir) {}
    public void smoothScrollTo(int x, int y) {}
    public void setFillViewport(boolean b) {}
    protected android.view.ViewGroup.LayoutParams generateDefaultLayoutParams() { return new LayoutParams(LayoutParams.MATCH_PARENT, LayoutParams.WRAP_CONTENT); }
    protected String idoridType() { return "ScrollView"; }
}
