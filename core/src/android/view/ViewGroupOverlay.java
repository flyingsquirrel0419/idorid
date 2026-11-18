package android.view;
import java.util.ArrayList;
public class ViewGroupOverlay extends ViewOverlay { private final ArrayList<View> views=new ArrayList<View>(); public void add(View v){if(v!=null&&!views.contains(v))views.add(v);} public void remove(View v){views.remove(v);} public void clear(){super.clear();views.clear();} }
