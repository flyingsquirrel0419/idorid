package android.view.accessibility;
public class AccessibilityManager {
    public interface TouchExplorationStateChangeListener { void onTouchExplorationStateChanged(boolean enabled); }
    public interface AccessibilityStateChangeListener { void onAccessibilityStateChanged(boolean enabled); }
    public java.util.List getEnabledAccessibilityServiceList(int feedbackType){return new java.util.ArrayList();}
    public boolean isEnabled(){return false;} public boolean isTouchExplorationEnabled(){return false;}
    public boolean addTouchExplorationStateChangeListener(TouchExplorationStateChangeListener l){return true;} public boolean removeTouchExplorationStateChangeListener(TouchExplorationStateChangeListener l){return true;}
    public boolean addAccessibilityStateChangeListener(AccessibilityStateChangeListener l){return true;} public boolean removeAccessibilityStateChangeListener(AccessibilityStateChangeListener l){return true;} public void sendAccessibilityEvent(AccessibilityEvent event){}
}
