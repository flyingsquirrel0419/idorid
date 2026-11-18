package android.view;
import android.graphics.drawable.Drawable;
public interface ContextMenu extends Menu { interface ContextMenuInfo {} void clearHeader(); ContextMenu setHeaderTitle(int id); ContextMenu setHeaderTitle(CharSequence title); ContextMenu setHeaderIcon(int id); ContextMenu setHeaderIcon(Drawable icon); ContextMenu setHeaderView(View view); }
