package android.view;
public interface MenuItem {
    int getItemId();
    CharSequence getTitle();
    MenuItem setTitle(CharSequence t);
    MenuItem setVisible(boolean v);
    MenuItem setEnabled(boolean e);
}
