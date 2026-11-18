package android.animation;
import android.view.View;
import android.view.ViewGroup;
public class LayoutTransition {
    public static final int CHANGE_APPEARING=0,CHANGE_DISAPPEARING=1,APPEARING=2,DISAPPEARING=3,CHANGING=4;
    public interface TransitionListener { void startTransition(LayoutTransition transition,ViewGroup container,View view,int type); void endTransition(LayoutTransition transition,ViewGroup container,View view,int type); }
    public void enableTransitionType(int type) {} public void disableTransitionType(int type) {} public boolean isTransitionTypeEnabled(int type){return false;} public boolean isRunning(){return false;} public boolean isChangingLayout(){return false;}
    public void setAnimator(int type,Animator animator) {} public Animator getAnimator(int type){return null;} public void setDuration(long duration) {} public void setDuration(int type,long duration) {} public void addTransitionListener(TransitionListener listener) {} public void removeTransitionListener(TransitionListener listener) {}
}
