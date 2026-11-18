package android.view.animation;
public class AccelerateInterpolator extends BaseInterpolator {
    private final float factor; public AccelerateInterpolator() { this(1); } public AccelerateInterpolator(float factor) { this.factor=factor; }
    public float getInterpolation(float input) { return factor==1 ? input*input : (float)Math.pow(input,2*factor); }
}
