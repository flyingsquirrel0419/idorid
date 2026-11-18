package java.lang;
public final class StackTraceElement implements java.io.Serializable {
    private final String declaringClass, methodName, fileName;
    private final int lineNumber;
    public StackTraceElement(String cls, String method, String file, int line) {
        declaringClass = cls; methodName = method; fileName = file; lineNumber = line;
    }
    public String getClassName() { return declaringClass; }
    public String getMethodName() { return methodName; }
    public String getFileName() { return fileName; }
    public int getLineNumber() { return lineNumber; }
    public boolean isNativeMethod() { return lineNumber == -2; }
    public String toString() {
        return declaringClass + "." + methodName + "(" + (lineNumber == -2 ? "Native Method" : fileName == null ? "Unknown Source"
            : lineNumber >= 0 ? fileName + ":" + lineNumber : fileName) + ")";
    }
}
