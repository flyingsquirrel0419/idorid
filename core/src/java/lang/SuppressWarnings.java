package java.lang;
import java.lang.annotation.*;
@Retention(RetentionPolicy.RUNTIME)
public @interface SuppressWarnings { String[] value(); }
