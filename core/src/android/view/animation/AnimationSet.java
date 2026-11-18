package android.view.animation;
import java.util.ArrayList;
import java.util.List;
public class AnimationSet extends Animation { private final ArrayList<Animation> animations=new ArrayList<Animation>(); public AnimationSet(boolean shareInterpolator) {} public void addAnimation(Animation a) { animations.add(a); } public List<Animation> getAnimations() { return animations; } }
