package android.view;
import android.graphics.Outline;
public abstract class ViewOutlineProvider { public static final ViewOutlineProvider BACKGROUND=new ViewOutlineProvider(){public void getOutline(View v,Outline o){}}; public static final ViewOutlineProvider BOUNDS=new ViewOutlineProvider(){public void getOutline(View v,Outline o){o.setRect(0,0,v.getWidth(),v.getHeight());}}; public abstract void getOutline(View view,Outline outline); }
