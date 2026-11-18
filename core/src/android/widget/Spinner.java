package android.widget;

import android.content.Context;
import android.util.AttributeSet;
import android.view.Json;
import android.view.View;

/** Rendered by the host as a menu button; item choice comes back as a click on a row. */
public class Spinner extends AdapterView<SpinnerAdapter> {
    private SpinnerAdapter adapter;
    private int selected = -1;
    public Spinner(Context c) { super(c); }
    public Spinner(Context c, AttributeSet a) { super(c, a); }
    public Spinner(Context c, AttributeSet a, int s) { super(c, a, s); }
    public SpinnerAdapter getAdapter() { return adapter; }
    public void setAdapter(SpinnerAdapter a) {
        if (adapter != null) adapter.unregisterDataSetObserver(observer);
        adapter = a;
        if (a != null) a.registerDataSetObserver(observer);
        rebuild();
        if (a != null && a.getCount() > 0 && selected < 0) setSelection(0);
    }
    public void setSelection(int p) {
        if (adapter == null || p < 0 || p >= adapter.getCount()) return;
        boolean changed = p != selected;
        selected = p;
        if (changed && itemSelected != null) itemSelected.onItemSelected(this, getChildAt(p), p, adapter.getItemId(p));
    }
    public void setSelection(int p, boolean animate) { setSelection(p); }
    public int getSelectedItemPosition() { return selected; }
    public Object getSelectedItem() { return selected < 0 || adapter == null ? null : adapter.getItem(selected); }
    public long getSelectedItemId() { return selected < 0 || adapter == null ? INVALID_ROW_ID : adapter.getItemId(selected); }
    public View getSelectedView() { return selected < 0 ? null : getChildAt(selected); }
    public void setPrompt(CharSequence p) {}
    void onRowClicked(View row, int pos) { setSelection(pos); }
    protected void onMeasure(int ws, int hs) {
        float w = 0;
        for (int i = 0; i < getChildCount(); i++) {
            View c = getChildAt(i);
            CharSequence t = c instanceof TextView ? ((TextView) c).getText() : "";
            w = Math.max(w, android.view.VMText.width(t.toString(), 16f, 0));
        }
        int width = resolveSize(Math.max(getSuggestedMinimumWidth(), (int) Math.ceil(w) + 48 + getPaddingLeft() + getPaddingRight()), ws);
        int height = resolveSize(Math.max(getSuggestedMinimumHeight(), 48), hs);
        setMeasuredDimension(width, height);
    }
    protected void onLayout(boolean changed, int l, int t, int r, int b) {}
    protected String idoridType() { return "Spinner"; }
    protected void idoridProps(StringBuilder sb) {
        Json.key(sb, "selected"); sb.append(selected);
        Json.key(sb, "items"); sb.append('[');
        for (int i = 0; i < getChildCount(); i++) {
            if (i > 0) sb.append(',');
            View c = getChildAt(i);
            sb.append("{\"id\":").append(c.idoridUid).append(",\"text\":");
            CharSequence t = c instanceof TextView ? ((TextView) c).getText() : String.valueOf(adapter.getItem(i));
            Json.str(sb, t);
            sb.append('}');
        }
        sb.append(']');
    }
}
