package android.view;
import android.animation.TimeInterpolator;
import android.graphics.Insets;
import java.util.List;
public final class WindowInsetsAnimation {
    private final int type; private final long duration; private final TimeInterpolator interpolator; private float fraction;
    public WindowInsetsAnimation(int typeMask,TimeInterpolator interpolator,long durationMillis){type=typeMask;this.interpolator=interpolator;duration=durationMillis;}
    public WindowInsetsAnimation(int typeMask,android.view.animation.Interpolator interpolator,long durationMillis){this(typeMask,(TimeInterpolator)interpolator,durationMillis);}
    public int getTypeMask(){return type;} public long getDurationMillis(){return duration;} public float getFraction(){return fraction;} public void setFraction(float f){fraction=f;} public float getInterpolatedFraction(){return interpolator==null?fraction:interpolator.getInterpolation(fraction);}
    public static final class Bounds { private final Insets lower,upper; public Bounds(Insets lower,Insets upper){this.lower=lower;this.upper=upper;} public Insets getLowerBound(){return lower;} public Insets getUpperBound(){return upper;} }
    public abstract static class Callback { private static int INITIALIZED; static { INITIALIZED=1; } public static final int DISPATCH_MODE_STOP=0,DISPATCH_MODE_CONTINUE_ON_SUBTREE=1; private final int mode; public Callback(int mode){this.mode=mode;} public int getDispatchMode(){return mode;} public void onPrepare(WindowInsetsAnimation animation){} public Bounds onStart(WindowInsetsAnimation animation,Bounds bounds){return bounds;} public abstract WindowInsets onProgress(WindowInsets insets,List<WindowInsetsAnimation> running); public void onEnd(WindowInsetsAnimation animation){} }
}
