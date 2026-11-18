package android.view.accessibility;
import android.os.Bundle;
import java.util.List;
public class AccessibilityNodeProvider { public AccessibilityNodeInfo createAccessibilityNodeInfo(int id){return null;} public List<AccessibilityNodeInfo> findAccessibilityNodeInfosByText(String text,int id){return null;} public AccessibilityNodeInfo findFocus(int focus){return null;} public boolean performAction(int id,int action,Bundle args){return false;} }
