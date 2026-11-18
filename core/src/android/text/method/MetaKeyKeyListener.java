package android.text.method;
import android.text.Editable;
import android.view.KeyEvent;
import android.view.View;
public abstract class MetaKeyKeyListener implements KeyListener { public static void adjustMetaAfterKeypress(android.text.Spannable text){} public boolean onKeyDown(View v,Editable e,int code,KeyEvent event){return false;} public boolean onKeyUp(View v,Editable e,int code,KeyEvent event){return false;} public boolean onKeyOther(View v,Editable e,KeyEvent event){return false;} public void clearMetaKeyState(View v,Editable e,int states){} }
