package android.widget;
import android.content.Context;
import android.util.AttributeSet;
public class HorizontalScrollView extends FrameLayout {
    public HorizontalScrollView(Context c) { super(c); }
    public HorizontalScrollView(Context c, AttributeSet a) { super(c, a); }
    public HorizontalScrollView(Context c, AttributeSet a, int s) { super(c, a, s); }
    public void fullScroll(int dir) {}
    public void smoothScrollTo(int x, int y) {}
    protected String idoridType() { return "HorizontalScrollView"; }
}
