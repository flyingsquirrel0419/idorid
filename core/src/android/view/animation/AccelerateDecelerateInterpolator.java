package android.view.animation;
public class AccelerateDecelerateInterpolator extends BaseInterpolator { public float getInterpolation(float input) { return (float)(Math.cos((input+1)*Math.PI)/2.0)+0.5f; } }
