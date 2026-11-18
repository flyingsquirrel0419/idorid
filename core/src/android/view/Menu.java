package android.view;
public interface Menu {
    MenuItem add(CharSequence title);
    MenuItem add(int groupId, int itemId, int order, CharSequence title);
    MenuItem findItem(int id);
    int size();
    void clear();
}
