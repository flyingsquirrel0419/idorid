package android.view;
import android.graphics.Rect;
public final class FocusFinder { private static final FocusFinder INSTANCE=new FocusFinder(); public static FocusFinder getInstance(){return INSTANCE;} public View findNextFocus(ViewGroup root,View focused,int direction){return null;} public View findNextFocusFromRect(ViewGroup root,Rect focusedRect,int direction){return null;} }
