package android.view;
import android.content.Context;
public abstract class ActionProvider {
    public interface VisibilityListener { void onActionProviderVisibilityChanged(boolean visible); }
    private VisibilityListener listener; protected final Context context;
    public ActionProvider(Context context){this.context=context;} public abstract View onCreateActionView(); public View onCreateActionView(MenuItem item){return onCreateActionView();} public boolean onPerformDefaultAction(){return false;} public boolean hasSubMenu(){return false;} public void onPrepareSubMenu(SubMenu subMenu){} public boolean overridesItemVisibility(){return false;} public boolean isVisible(){return true;} public void setVisibilityListener(VisibilityListener l){listener=l;} public void refreshVisibility(){if(listener!=null&&overridesItemVisibility())listener.onActionProviderVisibilityChanged(isVisible());}
}
