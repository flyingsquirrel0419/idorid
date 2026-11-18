package android.text.method;
import android.view.View;
import android.graphics.Rect;
public interface TransformationMethod { CharSequence getTransformation(CharSequence source,View view); void onFocusChanged(View view,CharSequence source,boolean focused,int direction,Rect previouslyFocusedRect); }
