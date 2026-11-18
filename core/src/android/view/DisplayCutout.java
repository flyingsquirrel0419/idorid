package android.view;
import android.graphics.Rect;
import java.util.ArrayList;
import java.util.List;
public final class DisplayCutout { private final Rect safe; private final List<Rect> bounds; public DisplayCutout(Rect safeInsets,List<Rect> boundingRects){safe=safeInsets==null?new Rect():new Rect(safeInsets);bounds=boundingRects==null?new ArrayList<Rect>():new ArrayList<Rect>(boundingRects);} public int getSafeInsetLeft(){return safe.left;} public int getSafeInsetTop(){return safe.top;} public int getSafeInsetRight(){return safe.right;} public int getSafeInsetBottom(){return safe.bottom;} public List<Rect> getBoundingRects(){return new ArrayList<Rect>(bounds);} }
