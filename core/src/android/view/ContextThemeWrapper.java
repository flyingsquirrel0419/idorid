package android.view;
import android.content.Context;
import android.content.ContextWrapper;
import android.content.res.Configuration;
import android.content.res.Resources;
public class ContextThemeWrapper extends ContextWrapper {
    private int themeResId;
    public ContextThemeWrapper(){super(null);} public ContextThemeWrapper(Context base,int themeResId){super(base);this.themeResId=themeResId;}
    public ContextThemeWrapper(Context base,Resources.Theme theme){super(base);}
    public Resources.Theme getTheme(){return getBaseContext().getTheme();} public Resources getResources(){return getBaseContext().getResources();}
    public void setTheme(int id){themeResId=id;} public int getThemeResId(){return themeResId;} public void applyOverrideConfiguration(Configuration configuration){}
}
