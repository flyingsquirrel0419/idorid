package android.widget;
import android.content.Context;
import android.util.AttributeSet;
import android.view.View;
public class ListView extends AbsListView {
    public ListView(Context c) { super(c); }
    public ListView(Context c, AttributeSet a) { this(c, a, 16842868 /*listViewStyle*/); }
    public ListView(Context c, AttributeSet a, int s) { super(c, a, s); }
    public void addHeaderView(View v) {}
    public void addFooterView(View v) {}
    public void setDivider(android.graphics.drawable.Drawable d) { divider=d; }
    public void setDividerHeight(int h) { dividerHeight=h; }
    public void setItemsCanFocus(boolean b) {}
    private android.graphics.drawable.Drawable divider; private int dividerHeight;
    public ListView(Context c,AttributeSet a,int s,int r) { super(c,a,s,r); }
    public void addHeaderView(View v,Object data,boolean selectable) { addHeaderView(v); } public android.graphics.drawable.Drawable getDivider() { return divider; } public int getDividerHeight() { return dividerHeight; }

}
