package android.widget;

import android.content.Context;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collection;
import java.util.Comparator;
import java.util.List;

public class ArrayAdapter<T> extends BaseAdapter implements Filterable {
    private final Context ctx;
    private final int resource;
    private int dropDownResource;
    private final int fieldId;
    private final List<T> objects;
    private boolean notify = true;
    public ArrayAdapter(Context c, int resource) { this(c, resource, 0, new ArrayList<T>()); }
    public ArrayAdapter(Context c, int resource, int textViewId) { this(c, resource, textViewId, new ArrayList<T>()); }
    public ArrayAdapter(Context c, int resource, T[] objs) { this(c, resource, 0, new ArrayList<T>(Arrays.asList(objs))); }
    public ArrayAdapter(Context c, int resource, int textViewId, T[] objs) { this(c, resource, textViewId, new ArrayList<T>(Arrays.asList(objs))); }
    public ArrayAdapter(Context c, int resource, List<T> objs) { this(c, resource, 0, objs); }
    public ArrayAdapter(Context c, int resource, int textViewId, List<T> objs) { ctx = c; this.resource = resource; dropDownResource = resource; fieldId = textViewId; objects = objs; }
    public static ArrayAdapter<CharSequence> createFromResource(Context c, int arrayId, int resource) {
        return new ArrayAdapter<CharSequence>(c, resource, 0, new ArrayList<CharSequence>(Arrays.asList(c.getResources().getTextArray(arrayId))));
    }
    public Context getContext() { return ctx; }
    public void add(T o) { objects.add(o); changed(); }
    public void addAll(Collection<? extends T> c) { objects.addAll(c); changed(); }
    @SafeVarargs public final void addAll(T... items) { objects.addAll(Arrays.asList(items)); changed(); }
    public void insert(T o, int i) { objects.add(i, o); changed(); }
    public void remove(T o) { objects.remove(o); changed(); }
    public void clear() { objects.clear(); changed(); }
    public void sort(Comparator<? super T> c) { java.util.Collections.sort(objects, c); changed(); }
    public void setNotifyOnChange(boolean n) { notify = n; }
    private void changed() { if (notify) notifyDataSetChanged(); }
    public int getCount() { return objects.size(); }
    public T getItem(int p) { return objects.get(p); }
    public int getPosition(T item) { return objects.indexOf(item); }
    public long getItemId(int p) { return p; }
    public void setDropDownViewResource(int r) { dropDownResource = r; }
    public Filter getFilter() { return null; }
    public View getView(int position, View convertView, ViewGroup parent) { return create(position, convertView, parent, resource); }
    public View getDropDownView(int position, View convertView, ViewGroup parent) { return create(position, convertView, parent, dropDownResource); }
    private View create(int position, View convertView, ViewGroup parent, int res) {
        View v = convertView;
        if (v == null) {
            v = VMSimpleLayouts.isBuiltin(res) ? VMSimpleLayouts.create(ctx, res) : LayoutInflater.from(ctx).inflate(res, parent, false);
        }
        TextView text;
        if (fieldId == 0) text = (TextView) v;
        else text = (TextView) v.findViewById(fieldId);
        T item = getItem(position);
        text.setText(item instanceof CharSequence ? (CharSequence) item : String.valueOf(item));
        return v;
    }
}
