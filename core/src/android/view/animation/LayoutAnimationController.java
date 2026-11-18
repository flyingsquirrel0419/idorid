package android.view.animation;
public class LayoutAnimationController { private Animation animation; public LayoutAnimationController(Animation a) { animation=a; } public LayoutAnimationController(Animation a,float delay) { animation=a; } public Animation getAnimation() { return animation; } public void setAnimation(Animation a) { animation=a; } }
