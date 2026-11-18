package android.view;
import android.graphics.drawable.Drawable;
public interface SubMenu extends Menu { void clearHeader(); SubMenu setHeaderTitle(CharSequence title); SubMenu setHeaderTitle(int id); SubMenu setHeaderIcon(Drawable icon); SubMenu setHeaderIcon(int id); SubMenu setHeaderView(View view); SubMenu setIcon(Drawable icon); SubMenu setIcon(int id); MenuItem getItem(); }
