package android.view;
import android.content.Intent;
import android.content.res.ColorStateList;
import android.graphics.PorterDuff;
import android.graphics.drawable.Drawable;
public interface MenuItem {
    interface OnMenuItemClickListener { boolean onMenuItemClick(MenuItem item); }
    interface OnActionExpandListener { boolean onMenuItemActionExpand(MenuItem item); boolean onMenuItemActionCollapse(MenuItem item); }
    int getItemId(); int getGroupId(); int getOrder(); CharSequence getTitle(); CharSequence getTitleCondensed();
    MenuItem setTitle(CharSequence t); MenuItem setTitle(int t); MenuItem setTitleCondensed(CharSequence t);
    MenuItem setVisible(boolean v); boolean isVisible(); MenuItem setEnabled(boolean e); boolean isEnabled();
    MenuItem setCheckable(boolean v); boolean isCheckable(); MenuItem setChecked(boolean v); boolean isChecked();
    MenuItem setIcon(int i); MenuItem setIcon(Drawable d); Drawable getIcon(); MenuItem setIconTintList(ColorStateList t); ColorStateList getIconTintList(); MenuItem setIconTintMode(PorterDuff.Mode m); PorterDuff.Mode getIconTintMode();
    MenuItem setIntent(Intent intent); Intent getIntent(); MenuItem setAlphabeticShortcut(char c); MenuItem setAlphabeticShortcut(char c,int modifiers); char getAlphabeticShortcut(); int getAlphabeticModifiers(); MenuItem setNumericShortcut(char c); MenuItem setNumericShortcut(char c,int modifiers); char getNumericShortcut(); int getNumericModifiers(); MenuItem setShortcut(char n,char a); MenuItem setShortcut(char n,char a,int nm,int am);
    MenuItem setContentDescription(CharSequence c); CharSequence getContentDescription(); MenuItem setTooltipText(CharSequence t); CharSequence getTooltipText();
    MenuItem setActionView(View v); MenuItem setActionView(int id); View getActionView(); boolean expandActionView(); boolean collapseActionView(); boolean isActionViewExpanded(); MenuItem setOnActionExpandListener(OnActionExpandListener l); MenuItem setOnMenuItemClickListener(OnMenuItemClickListener l);
    void setShowAsAction(int action); MenuItem setShowAsActionFlags(int action); boolean hasSubMenu(); SubMenu getSubMenu(); ContextMenu.ContextMenuInfo getMenuInfo();
}
