package android.widget;
import android.database.DataSetObserver;
import android.view.View;
import android.view.ViewGroup;
import java.util.ArrayList;
public abstract class BaseAdapter implements ListAdapter, SpinnerAdapter {
    private final ArrayList<DataSetObserver> observers = new ArrayList<DataSetObserver>();
    public void registerDataSetObserver(DataSetObserver o) { observers.add(o); }
    public void unregisterDataSetObserver(DataSetObserver o) { observers.remove(o); }
    public void notifyDataSetChanged() { for (DataSetObserver o : new ArrayList<DataSetObserver>(observers)) o.onChanged(); }
    public void notifyDataSetInvalidated() { for (DataSetObserver o : new ArrayList<DataSetObserver>(observers)) o.onInvalidated(); }
    public boolean hasStableIds() { return false; }
    public boolean areAllItemsEnabled() { return true; }
    public boolean isEnabled(int p) { return true; }
    public View getDropDownView(int p, View c, ViewGroup parent) { return getView(p, c, parent); }
    public int getItemViewType(int p) { return 0; }
    public int getViewTypeCount() { return 1; }
    public boolean isEmpty() { return getCount() == 0; }
}
