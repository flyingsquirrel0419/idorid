package android.text.method;
public class PasswordTransformationMethod implements TransformationMethod {
    public CharSequence getTransformation(CharSequence source,android.view.View view) { StringBuilder out=new StringBuilder(); for(int i=0;i<source.length();i++)out.append('\u2022'); return out.toString(); }
    public void onFocusChanged(android.view.View view,CharSequence source,boolean focused,int direction,android.graphics.Rect rect) {}
    private static final PasswordTransformationMethod I = new PasswordTransformationMethod();
    public static PasswordTransformationMethod getInstance() { return I; }
}
