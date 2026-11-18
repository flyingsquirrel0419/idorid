package android.view;
import android.graphics.drawable.Drawable;
import java.util.ArrayList;
public class ViewOverlay { private final ArrayList<Drawable> drawables=new ArrayList<Drawable>(); public void add(Drawable d){if(d!=null&&!drawables.contains(d))drawables.add(d);} public void remove(Drawable d){drawables.remove(d);} public void clear(){drawables.clear();} }
