package android.widget;

import android.content.Context;
import android.database.DataSetObserver;
import android.util.AttributeSet;
import android.view.View;
import android.view.ViewGroup;

public abstract class AdapterView<T extends Adapter> extends ViewGroup {
    public static final int INVALID_POSITION = -1;
    public static final long INVALID_ROW_ID = Long.MIN_VALUE;
    public interface OnItemClickListener { void onItemClick(AdapterView<?> parent, View view, int position, long id); }
    public interface OnItemLongClickListener { boolean onItemLongClick(AdapterView<?> parent, View view, int position, long id); }
    public interface OnItemSelectedListener { void onItemSelected(AdapterView<?> parent, View view, int position, long id); void onNothingSelected(AdapterView<?> parent); }
    OnItemClickListener itemClick;
    OnItemLongClickListener itemLongClick;
    OnItemSelectedListener itemSelected;
    private View emptyView;
    final DataSetObserver observer = new DataSetObserver() { public void onChanged() { rebuild(); } public void onInvalidated() { rebuild(); } };
    public AdapterView(Context c) { super(c); }
    public AdapterView(Context c, AttributeSet a) { super(c, a); }
    public AdapterView(Context c, AttributeSet a, int s) { super(c, a, s); }
    public abstract T getAdapter();
    public abstract void setAdapter(T adapter);
    public void setOnItemClickListener(OnItemClickListener l) { itemClick = l; }
    public final OnItemClickListener getOnItemClickListener() { return itemClick; }
    public void setOnItemLongClickListener(OnItemLongClickListener l) { itemLongClick = l; }
    public void setOnItemSelectedListener(OnItemSelectedListener l) { itemSelected = l; }
    public final OnItemSelectedListener getOnItemSelectedListener() { return itemSelected; }
    public boolean performItemClick(View view, int position, long id) {
        if (itemClick == null) return false;
        itemClick.onItemClick(this, view, position, id);
        return true;
    }
    public Object getItemAtPosition(int p) { T a = getAdapter(); return a == null || p < 0 ? null : a.getItem(p); }
    public long getItemIdAtPosition(int p) { T a = getAdapter(); return a == null || p < 0 ? INVALID_ROW_ID : a.getItemId(p); }
    public int getCount() { T a = getAdapter(); return a == null ? 0 : a.getCount(); }
    public int getPositionForView(View v) {
        for (int i = 0; i < getChildCount(); i++) { View c = getChildAt(i); for (View x = v; x != null; x = x.getParent() instanceof View ? (View) x.getParent() : null) if (x == c) return i; }
        return INVALID_POSITION;
    }
    public void setEmptyView(View v) { emptyView = v; rebuild(); }
    public View getEmptyView() { return emptyView; }
    public int getFirstVisiblePosition() { return 0; }
    public int getLastVisiblePosition() { return getCount() - 1; }
    public void setSelection(int p) {}
    /** Materializes all adapter rows as children (no recycling yet). */
    void rebuild() {
        removeAllViews();
        T a = getAdapter();
        int n = a == null ? 0 : a.getCount();
        if (emptyView != null) emptyView.setVisibility(n == 0 ? VISIBLE : GONE);
        for (int i = 0; i < n; i++) {
            View v = a.getView(i, null, this);
            if (v.getParent() != null) ((ViewGroup) v.getParent()).removeView(v);
            final int pos = i;
            final View row = v;
            addView(v, new ViewGroup.LayoutParams(ViewGroup.LayoutParams.MATCH_PARENT, ViewGroup.LayoutParams.WRAP_CONTENT));
            if (itemClick != null || itemLongClick != null || this instanceof Spinner) {
                if (!(v instanceof Button) && !v.hasOnClickListeners()) v.setOnClickListener(new OnClickListener() {
                    public void onClick(View x) { onRowClicked(row, pos); }
                });
            }
        }
    }
    void onRowClicked(View row, int pos) { performItemClick(row, pos, getItemIdAtPosition(pos)); }
    public AdapterView(Context c,AttributeSet a,int s,int r) { super(c,a,s,r); }
    public int getSelectedItemPosition() { return INVALID_POSITION; } public Object getSelectedItem() { return getItemAtPosition(getSelectedItemPosition()); } public long getSelectedItemId() { return getItemIdAtPosition(getSelectedItemPosition()); } public View getSelectedView() { return null; }

}
