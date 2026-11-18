package android.view.accessibility;
import java.util.ArrayList;
public class AccessibilityEvent extends AccessibilityRecord {
    private int eventType,contentChangeTypes; private CharSequence packageName; private final ArrayList<AccessibilityRecord> records=new ArrayList<AccessibilityRecord>();
    public static AccessibilityEvent obtain(){return new AccessibilityEvent();} public static AccessibilityEvent obtain(int type){AccessibilityEvent e=new AccessibilityEvent();e.eventType=type;return e;}
    public int getEventType(){return eventType;} public void setEventType(int t){eventType=t;} public void setPackageName(CharSequence p){packageName=p;} public CharSequence getPackageName(){return packageName;} public void setContentChangeTypes(int t){contentChangeTypes=t;} public int getContentChangeTypes(){return contentChangeTypes;}
    public void appendRecord(AccessibilityRecord r){records.add(r);} public int getRecordCount(){return records.size();} public AccessibilityRecord getRecord(int i){return records.get(i);} public void recycle(){}
}
