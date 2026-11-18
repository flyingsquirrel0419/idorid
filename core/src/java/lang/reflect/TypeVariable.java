package java.lang.reflect;

public interface TypeVariable<D> extends Type {
    Type[] getBounds();
    D getGenericDeclaration();
    String getName();
}
