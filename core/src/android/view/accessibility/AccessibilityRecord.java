package android.view.accessibility;
import android.view.View;
import java.util.ArrayList;
import java.util.List;
public class AccessibilityRecord {
    private CharSequence description,className; private final ArrayList<CharSequence> text=new ArrayList<CharSequence>(); private int itemCount,currentIndex,fromIndex,toIndex,scrollX,scrollY,maxScrollX,maxScrollY; private boolean scrollable;
    private boolean checked,enabled,password; public void setChecked(boolean v){checked=v;} public boolean isChecked(){return checked;} public void setEnabled(boolean v){enabled=v;} public boolean isEnabled(){return enabled;} public void setPassword(boolean v){password=v;} public boolean isPassword(){return password;}
    public void setClassName(CharSequence s){className=s;} public CharSequence getClassName(){return className;} public void setContentDescription(CharSequence s){description=s;} public CharSequence getContentDescription(){return description;} public List<CharSequence> getText(){return text;}
    public void setItemCount(int n){itemCount=n;} public int getItemCount(){return itemCount;} public void setCurrentItemIndex(int n){currentIndex=n;} public int getCurrentItemIndex(){return currentIndex;} public void setFromIndex(int n){fromIndex=n;} public int getFromIndex(){return fromIndex;} public void setToIndex(int n){toIndex=n;} public int getToIndex(){return toIndex;}
    public void setScrollX(int n){scrollX=n;} public int getScrollX(){return scrollX;} public void setScrollY(int n){scrollY=n;} public int getScrollY(){return scrollY;} public void setMaxScrollX(int n){maxScrollX=n;} public int getMaxScrollX(){return maxScrollX;} public void setMaxScrollY(int n){maxScrollY=n;} public int getMaxScrollY(){return maxScrollY;} public void setScrollable(boolean b){scrollable=b;} public boolean isScrollable(){return scrollable;} public void setSource(View v){} public void setSource(View v,int id){}
}
