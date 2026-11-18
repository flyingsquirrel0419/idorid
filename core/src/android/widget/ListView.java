package android.widget;
import android.content.Context;
import android.util.AttributeSet;
import android.view.View;
public class ListView extends AbsListView {
    public ListView(Context c) { super(c); }
    public ListView(Context c, AttributeSet a) { super(c, a); }
    public ListView(Context c, AttributeSet a, int s) { super(c, a, s); }
    public void addHeaderView(View v) {}
    public void addFooterView(View v) {}
    public void setDivider(android.graphics.drawable.Drawable d) {}
    public void setDividerHeight(int h) {}
    public void setItemsCanFocus(boolean b) {}
}
