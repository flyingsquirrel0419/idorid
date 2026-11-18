package android.view;
import android.graphics.Point;
import android.util.DisplayMetrics;
public class Display {
    public static final int DEFAULT_DISPLAY=0, ROTATION_0=0, ROTATION_90=1, ROTATION_180=2, ROTATION_270=3;
    public int getDisplayId(){return DEFAULT_DISPLAY;} public int getRotation(){return ROTATION_0;}
    public int getWidth(){return 390;} public int getHeight(){return 844;}
    public void getSize(Point out){out.set(getWidth(),getHeight());} public void getRealSize(Point out){getSize(out);}
    public void getMetrics(DisplayMetrics out){out.widthPixels=getWidth();out.heightPixels=getHeight();} public void getRealMetrics(DisplayMetrics out){getMetrics(out);}
    public float getRefreshRate(){return 60;}
}
