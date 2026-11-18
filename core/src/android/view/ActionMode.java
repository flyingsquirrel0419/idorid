package android.view;
public abstract class ActionMode {
    public interface Callback { boolean onCreateActionMode(ActionMode mode,Menu menu); boolean onPrepareActionMode(ActionMode mode,Menu menu); boolean onActionItemClicked(ActionMode mode,MenuItem item); void onDestroyActionMode(ActionMode mode); }
    private Object tag; private boolean titleOptional;
    public abstract void setTitle(CharSequence title); public abstract void setTitle(int title); public abstract void setSubtitle(CharSequence subtitle); public abstract void setSubtitle(int subtitle); public abstract void setCustomView(View view); public abstract void invalidate(); public abstract void finish(); public abstract Menu getMenu(); public abstract CharSequence getTitle(); public abstract CharSequence getSubtitle(); public abstract View getCustomView(); public abstract MenuInflater getMenuInflater();
    public void setTag(Object t){tag=t;} public Object getTag(){return tag;} public void setTitleOptionalHint(boolean optional){titleOptional=optional;} public boolean getTitleOptionalHint(){return titleOptional;} public boolean isTitleOptional(){return false;}
}
