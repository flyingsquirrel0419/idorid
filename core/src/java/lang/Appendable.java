package java.lang;
public interface Appendable {
    Appendable append(CharSequence cs) throws java.io.IOException;
    Appendable append(CharSequence cs, int s, int e) throws java.io.IOException;
    Appendable append(char c) throws java.io.IOException;
}
