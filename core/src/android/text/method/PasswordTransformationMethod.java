package android.text.method;
public class PasswordTransformationMethod implements TransformationMethod {
    private static final PasswordTransformationMethod I = new PasswordTransformationMethod();
    public static PasswordTransformationMethod getInstance() { return I; }
}
