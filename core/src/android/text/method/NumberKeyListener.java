package android.text.method;
import android.text.InputFilter;
import android.text.Spanned;
public abstract class NumberKeyListener extends MetaKeyKeyListener implements InputFilter { protected abstract char[] getAcceptedChars(); public CharSequence filter(CharSequence source,int start,int end,Spanned dest,int dstart,int dend){char[] accepted=getAcceptedChars();StringBuilder result=new StringBuilder();boolean changed=false;for(int i=start;i<end;i++){char c=source.charAt(i);boolean ok=false;for(char a:accepted)if(a==c){ok=true;break;}if(ok)result.append(c);else changed=true;}return changed?result.toString():null;} }
