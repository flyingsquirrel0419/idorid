package android.widget;
import android.content.Context;
import android.util.AttributeSet;
public class GridView extends AbsListView {
    public GridView(Context c) { super(c); }
    public static final int AUTO_FIT = -1, NO_STRETCH = 0, STRETCH_SPACING = 1, STRETCH_COLUMN_WIDTH = 2;
    private int numColumns = 1, columnWidth, hSpace, vSpace;
    public GridView(Context c, AttributeSet a) {
        super(c, a);
        if (a instanceof android.view.XmlAttrs) {
            android.view.XmlAttrs x = (android.view.XmlAttrs) a;
            String n = x.string("numColumns");
            numColumns = n == null ? 1 : n.equals("auto_fit") ? AUTO_FIT : x.integer("numColumns", 1);
            columnWidth = (int) x.dimension("columnWidth", 0);
            hSpace = (int) x.dimension("horizontalSpacing", 0);
            vSpace = (int) x.dimension("verticalSpacing", 0);
        }
    }
    public void setNumColumns(int n) { numColumns = n; requestLayout(); }
    public int getNumColumns() { return numColumns; }
    public void setColumnWidth(int w) { columnWidth = w; requestLayout(); }
    public void setHorizontalSpacing(int s) { hSpace = s; requestLayout(); }
    public void setVerticalSpacing(int s) { vSpace = s; requestLayout(); }
    public void setStretchMode(int m) {}
    int columns(int width) {
        if (numColumns > 0) return numColumns;
        int cw = columnWidth > 0 ? columnWidth : 100;
        return Math.max(1, (width + hSpace) / (cw + hSpace));
    }
    int hSpacing() { return hSpace; }
    int vSpacing() { return vSpace; }
}
