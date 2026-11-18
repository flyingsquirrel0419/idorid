package android.text.method;
public class HideReturnsTransformationMethod implements TransformationMethod {
    public CharSequence getTransformation(CharSequence source,android.view.View view) { return source.toString().replace('\r','\ufeff'); }
    public void onFocusChanged(android.view.View view,CharSequence source,boolean focused,int direction,android.graphics.Rect rect) {}
    private static final HideReturnsTransformationMethod I = new HideReturnsTransformationMethod();
    public static HideReturnsTransformationMethod getInstance() { return I; }
}
