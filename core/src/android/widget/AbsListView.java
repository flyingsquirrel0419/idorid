package android.widget;
import android.content.Context;
import android.util.AttributeSet;
public abstract class AbsListView extends AdapterView<ListAdapter> {
    public static final int CHOICE_MODE_NONE = 0, CHOICE_MODE_SINGLE = 1, CHOICE_MODE_MULTIPLE = 2;
    public interface OnScrollListener { int SCROLL_STATE_IDLE = 0; void onScrollStateChanged(AbsListView v, int state); void onScroll(AbsListView v, int first, int visible, int total); }
    ListAdapter adapter;
    public AbsListView(Context c) { super(c); }
    public AbsListView(Context c, AttributeSet a) { super(c, a); }
    public AbsListView(Context c, AttributeSet a, int s) { super(c, a, s); }
    public ListAdapter getAdapter() { return adapter; }
    public void setAdapter(ListAdapter a) {
        if (adapter != null) adapter.unregisterDataSetObserver(observer);
        adapter = a;
        if (a != null) a.registerDataSetObserver(observer);
        rebuild();
    }
    public void setOnScrollListener(OnScrollListener l) {}
    public void setChoiceMode(int m) {}
    public void setItemChecked(int p, boolean v) {}
    public void setTextFilterEnabled(boolean b) {}
    public void setFastScrollEnabled(boolean b) {}
    public void smoothScrollToPosition(int p) {}
    public void setSelector(int r) {}
    public void setStackFromBottom(boolean b) {}
    public void setTranscriptMode(int m) {}
    public void invalidateViews() { rebuild(); }
    protected android.view.ViewGroup.LayoutParams generateDefaultLayoutParams() { return new LayoutParams(LayoutParams.MATCH_PARENT, LayoutParams.WRAP_CONTENT); }
    protected String idoridType() { return "ScrollView"; }
    protected void idoridProps(StringBuilder sb) {
        android.view.Json.key(sb, "children");
        sb.append("[{\"id\":0,\"type\":\"LinearLayout\",\"orientation\":\"vertical\",\"width\":\"match\",\"height\":\"wrap\"");
        super.idoridProps(sb);
        sb.append("}]");
    }
}
