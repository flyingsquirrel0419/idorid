package android.widget;

import android.content.Context;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import java.util.List;
import java.util.Map;

public class SimpleAdapter extends BaseAdapter implements Filterable {
    public interface ViewBinder { boolean setViewValue(View view,Object data,String textRepresentation); }
    private final Context context;
    private final List<? extends Map<String, ?>> data;
    private final int resource;
    private final String[] from;
    private final int[] to;
    private ViewBinder binder;
    public SimpleAdapter(Context c,List<? extends Map<String, ?>> data,int resource,String[] from,int[] to) { context=c; this.data=data; this.resource=resource; this.from=from; this.to=to; }
    public int getCount() { return data.size(); }
    public Object getItem(int position) { return data.get(position); }
    public long getItemId(int position) { return position; }
    public View getView(int position,View convertView,ViewGroup parent) {
        View view=convertView==null?LayoutInflater.from(context).inflate(resource,parent,false):convertView;
        Map<String,?> row=data.get(position);
        for(int i=0;i<from.length && i<to.length;i++) {
            View target=view.findViewById(to[i]); if(target==null) continue;
            Object value=row.get(from[i]); String text=value==null?"":value.toString();
            if(binder!=null && binder.setViewValue(target,value,text)) continue;
            if(target instanceof TextView) ((TextView)target).setText(text);
            else if(target instanceof Checkable && value instanceof Boolean) ((Checkable)target).setChecked(((Boolean)value).booleanValue());
            else if(target instanceof ImageView && value instanceof Number) ((ImageView)target).setImageResource(((Number)value).intValue());
        }
        return view;
    }
    public void setViewBinder(ViewBinder b) { binder=b; }
    public ViewBinder getViewBinder() { return binder; }
    public Filter getFilter() { return new Filter() {}; }
}
