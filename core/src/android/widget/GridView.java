package android.widget;
import android.content.Context;
import android.util.AttributeSet;
public class GridView extends AbsListView {
    public GridView(Context c) { super(c); }
    public GridView(Context c, AttributeSet a) { super(c, a); }
    public void setNumColumns(int n) {}
    public void setColumnWidth(int w) {}
    public void setHorizontalSpacing(int s) {}
    public void setVerticalSpacing(int s) {}
    public void setStretchMode(int m) {}
}
