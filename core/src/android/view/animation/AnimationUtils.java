package android.view.animation;
import android.content.Context;
public class AnimationUtils { public static Animation loadAnimation(Context c,int id) { return new Animation(); } public static Interpolator loadInterpolator(Context c,int id) { return new LinearInterpolator(); } public static long currentAnimationTimeMillis() { return android.os.SystemClock.uptimeMillis(); } }
