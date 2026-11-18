package android.view.animation;
public class DecelerateInterpolator extends BaseInterpolator {
    private final float factor; public DecelerateInterpolator() { this(1); } public DecelerateInterpolator(float factor) { this.factor=factor; }
    public float getInterpolation(float input) { return factor==1 ? 1-(1-input)*(1-input) : 1-(float)Math.pow(1-input,2*factor); }
}
