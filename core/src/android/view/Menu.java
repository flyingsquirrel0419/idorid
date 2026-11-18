package android.view;
public interface Menu {
    MenuItem add(CharSequence title); MenuItem add(int groupId,int itemId,int order,CharSequence title);
    SubMenu addSubMenu(int groupId,int itemId,int order,CharSequence title);
    MenuItem findItem(int id); MenuItem getItem(int index); void removeItem(int id); void setQwertyMode(boolean qwerty);
    int size(); void clear();
 default boolean performShortcut(int keyCode, KeyEvent event, int flags) { return false; }
}
